use std::collections::HashMap;
use crate::peripheral::Peripheral;

struct PeripheralsStore {
    peripherals: HashMap<String, Box<dyn Peripheral>>,
}

// TODO: Peripherals should attach their ID to the FunctionTemplates.
// TODO: The core apis should live within their own file.
// TODO: The js_api method of peripherals should take the peripheral id.
// TODO: Create proper macros for defining the js_api templates. (create a DSL language maybe).
// TODO: Create a macro DSL language for parsing js arguments.
// TODO: Use methods to minimize the repeated code in macros where possible (avoid inflating the binary size).

impl PeripheralsStore {
    // FIXME: Figure out dynamic typing for those methods.

    // TODO: Add a method for getting a list of all the registered peripherals IDs. (needed for core api)

    /// Gets a mutable reference to a peripheral.
    pub fn get(scope: &mut v8::HandleScope, id: String) -> &mut Box<dyn Peripheral> {
        let store = Self::get_store(scope);
        store.peripherals.get_mut(id.as_str()).unwrap()
    }

    /// Unmounts a peripheral from the peripherals store.
    pub fn unmount(scope: &mut v8::HandleScope, id: String) -> Box<dyn Peripheral> {
        let store = Self::get_store(scope);
        store.peripherals.remove(id.as_str()).unwrap()
    }

    /// Mounts a peripheral into the peripherals store.
    pub fn mount(scope: &mut v8::HandleScope, peripheral: Box<dyn Peripheral>) -> String {
        let store = Self::get_store(scope);
        let id = Self::generate_id();

        store.peripherals.insert(id.clone(), peripheral);
        id
    }

    /// Generates an ID for a new peripheral to be mounted.
    fn generate_id() -> String {
        todo!() // FIXME: Get a reliable UUID generation.
    }

    /// (internal) Gets the peripherals store for an isolate.
    /// Creates the peripherals store if it was not created before for the provided isolate.
    fn get_store(scope: &mut v8::HandleScope) -> &mut Self {
        let slot = scope.get_slot_mut::<PeripheralsStore>();

        match slot {
            Some(store) => store,
            None => Self::initialize_store(scope),
        }
    }

    /// (internal) Initializes the peripherals store for an isolate.
    /// Make sure that it has not been initialized before.
    ///
    /// Use `get()` instead as it automatically initializes the store.
    fn initialize_store(scope: &mut v8::HandleScope) -> &mut Self {
        let store = Self { peripherals: HashMap::new() };
        scope.set_slot(store);

        scope.get_slot_mut::<Self>().unwrap()
    }
}