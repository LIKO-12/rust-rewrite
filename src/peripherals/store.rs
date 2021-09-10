use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;
use std::string::String as RustString;

use v8::*;

use super::Peripheral;
use crate::add_method;

struct PeripheralEntry {
    type_name: &'static str,
    js_api: Global<ObjectTemplate>,
}

pub struct PeripheralEvent {
    /// The id of the peripheral which authored the event.
    pub peripheral_id: RustString,
    /// The name of the event.
    pub name: RustString,
    /// The data associated with the event.
    pub data: Option<Global<Value>>,
}

impl PeripheralEvent {
    pub fn js_object<'a>(self, scope: &mut HandleScope<'a>) -> Local<'a, Object> {
        let object = ObjectTemplate::new(scope);

        object.set(
            String::new(scope, "peripheralId").unwrap().into(),
            String::new(scope, self.peripheral_id.as_str()).unwrap().into(),
        );

        object.set(
            String::new(scope, "name").unwrap().into(),
            String::new(scope, self.name.as_str()).unwrap().into(),
        );

        if let Some(data) = self.data {
            object.set(
                String::new(scope, "data").unwrap().into(),
                Local::new(scope, data).into(),
            );
        }

        object.new_instance(scope).unwrap()
    }
}

pub struct PeripheralsStore {
    peripherals: HashMap<RustString, PeripheralEntry>,
    events_queue: VecDeque<PeripheralEvent>,
    /// The current active event listener promise (if there's a listener).
    listening_promise: Option<Global<PromiseResolver>>,
}

/// Throws a JS TypeError with a specific message, caller must return immediately after it.
fn throw_type_error(scope: &mut HandleScope, message: &str) {
    let message = String::new(scope, message).unwrap();
    let exception = Exception::type_error(scope, message);
    scope.throw_exception(exception);
}

/// Throws a JS Error with a specific message, caller must return immediately after it.
fn throw_error(scope: &mut HandleScope, message: &str) {
    let message = String::new(scope, message).unwrap();
    let exception = Exception::error(scope, message);
    scope.throw_exception(exception);
}

impl PeripheralsStore {
    /// Creates a new peripherals store.
    fn new() -> Self {
        Self {
            peripherals: HashMap::new(),
            events_queue: VecDeque::new(),
            listening_promise: None,
        }
    }

    /// Mounts a peripheral into a V8 isolate.
    pub fn mount(isolate: &mut Isolate, id: RustString, peripheral: Box<dyn Peripheral>) {
        let store_cell = Self::get(isolate);
        let mut store = store_cell.borrow_mut();

        let js_api: Global<ObjectTemplate> = {
            let scope = &mut HandleScope::new(isolate);
            let js_api = peripheral.js_api(scope);
            Global::new(scope, js_api)
        };

        store.peripherals.insert(id, PeripheralEntry {
            type_name: peripheral.type_name(),
            js_api,
        });
    }

    /// Gets the peripherals store allocated for a specific V8 isolate
    pub fn get(isolate: &mut Isolate) -> Rc<RefCell<PeripheralsStore>> {
        match isolate.get_slot::<Rc<RefCell<PeripheralsStore>>>() {
            Some(store) => store.clone(),
            None => {
                let store = Rc::new(RefCell::new(Self::new()));
                isolate.set_slot(store.clone());
                store
            }
        }
    }

    pub fn push_event(scope: &mut HandleScope, event: PeripheralEvent) {
        let store_cell = Self::get(scope);
        let mut store = store_cell.borrow_mut();

        if let Some(promise) = store.listening_promise.take() {
            let event_object = event.js_object(scope);
            promise.get(scope).resolve(scope, event_object.into());
        } else {
            store.events_queue.push_back(event);
        }
    }

    /// Provides the JavaScript api for interacting with the peripherals.
    pub fn js_api<'a>(scope: &'a mut HandleScope<()>) -> Local<'a, ObjectTemplate> {
        let api = ObjectTemplate::new(scope);

        add_method!(api, scope, "get", Self::get_callback);
        add_method!(api, scope, "getIds", Self::get_ids_callback);
        add_method!(api, scope, "getType", Self::get_type_callback);
        add_method!(api, scope, "pullEvent", Self::pull_event_callback);

        api
    }

    /// JS Callback, returns an instance of a peripheral's api.
    /// arguments: (String): The peripheral's id.
    fn get_callback(scope: &mut HandleScope, args: FunctionCallbackArguments, mut rv: ReturnValue) {
        if args.length() < 1 {
            throw_type_error(scope, "At least 1 argument required, but only 0 passed");
            return;
        }

        let id = args.get(0).to_string(scope).unwrap().to_rust_string_lossy(scope);
        let store_cell = PeripheralsStore::get(scope);

        let store = store_cell.borrow();
        let peripheral = store.peripherals.get(id.as_str());

        match peripheral {
            Some(peripheral) => {
                let result = peripheral.js_api.get(scope).new_instance(scope).unwrap();
                rv.set(result.into());
            }
            None => throw_error(scope, "Peripheral doesn't exist"),
        }
    }

    /// JS Callback, returns an array of mounted peripheral's ids.
    /// arguments: none.
    fn get_ids_callback(scope: &mut HandleScope, _args: FunctionCallbackArguments, mut rv: ReturnValue) {
        let store_cell = PeripheralsStore::get(scope);
        let store = store_cell.borrow();

        let ids: Vec<Local<Value>> = store.peripherals.keys()
            .map(|id| String::new(scope, id.as_str()).unwrap().into()).collect();

        rv.set(Array::new_with_elements(scope, ids.as_slice()).into());
    }

    /// JS Callback, returns the type of a specific peripheral.
    /// arguments: (String): The peripheral's id.
    fn get_type_callback(scope: &mut HandleScope, args: FunctionCallbackArguments, mut rv: ReturnValue) {
        if args.length() < 1 {
            throw_type_error(scope, "At least 1 argument required, but only 0 passed");
            return;
        }

        let id = args.get(0).to_string(scope).unwrap().to_rust_string_lossy(scope);
        let store_cell = PeripheralsStore::get(scope);

        let store = store_cell.borrow();
        let peripheral = store.peripherals.get(id.as_str());

        match peripheral {
            Some(peripheral) => {
                let result = String::new(scope, peripheral.type_name).unwrap();
                rv.set(result.into());
            }
            None => throw_error(scope, "Peripheral doesn't exist"),
        }
    }

    /// JS Callback, returns a promise which is resolved into a peripheral event object.
    /// arguments: (boolean/undefined): Whether to pull passively or not.
    ///
    /// When in passive mode, the promise is resolved instantly either with an event from the queue,
    /// or with `null` if the queue is empty.
    ///
    /// Otherwise in aggressive mode, the promise is not resolved until an event is pushed to the queue.
    fn pull_event_callback(scope: &mut HandleScope, args: FunctionCallbackArguments, mut rv: ReturnValue) {
        let promise_resolver = PromiseResolver::new(scope).unwrap();
        rv.set(promise_resolver.get_promise(scope).into());

        let passive = args.get(0).boolean_value(scope);

        let state_cell = Self::get(scope);
        let mut state = state_cell.borrow_mut();

        let event = state.events_queue.pop_front();

        if let Some(event) = event {
            let event_object = event.js_object(scope);
            promise_resolver.resolve(scope, event_object.into());
        } else if passive {
            let null = v8::null(scope);
            promise_resolver.resolve(scope, null.into());
        } else {
            let promise_resolver = Global::new(scope, promise_resolver);
            state.listening_promise.insert(promise_resolver);
        }
    }
}