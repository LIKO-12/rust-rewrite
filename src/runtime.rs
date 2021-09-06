use std::sync::Once;

use v8::*;
use crate::peripherals_store::PeripheralsStore;

/// Initializes the JavaScript V8 engine, should be only done once.
/// And that's done by the JsRuntime::new constructor.
fn initialize_v8() {
    V8::set_flags_from_string("--single-threaded");

    let platform = new_single_threaded_default_platform(false).make_shared();
    V8::initialize_platform(platform);
    V8::initialize();
}

pub struct JsRuntime {
    isolate: OwnedIsolate,
    context: Global<Context>,
}

impl JsRuntime {
    /// Creates a new JS runtime environment.
    pub fn new() -> Self {
        // Initialize V8 for only once.
        static JS_INIT: Once = Once::new();
        JS_INIT.call_once(initialize_v8);

        let mut isolate = Isolate::new(CreateParams::default());

        let context: Global<Context> = {
            let scope = &mut HandleScope::new(&mut isolate);
            let context = {
                let globals = ObjectTemplate::new(scope);

                globals.set(
                    String::new(scope, "log").unwrap().into(),
                    FunctionTemplate::new(scope, Self::log_callback).into(),
                );

                globals.set(
                    String::new(scope, "peripherals").unwrap().into(),
                    PeripheralsStore::js_api(scope).into(),
                );

                Context::new_from_template(scope, globals)
            };

            Global::new(scope, context)
        };

        JsRuntime {
            isolate,
            context,
        }
    }

    /// Get a mutable reference to the V8's isolate.
    pub fn isolate(&mut self) -> &mut Isolate {
        &mut self.isolate
    }

    /// Get a HandleScope with the runtime isolate and global context.
    pub fn handle_scope(&mut self) -> HandleScope {
        HandleScope::with_context(&mut self.isolate, self.context.clone())
    }

    /// Execute arbitrary JS code under the runtime environment.
    pub fn execute(&mut self, source: &str) {
        let scope = &mut HandleScope::with_context(&mut self.isolate, self.context.clone());

        let code = String::new(scope, source).unwrap();
        let script = Script::compile(scope, code, None).unwrap();

        script.run(scope);
    }

    /// JS Callback, takes a single value as an argument, and logs it into stdout.
    fn log_callback(scope: &mut HandleScope, args: FunctionCallbackArguments, _rv: ReturnValue) {
        let message = args.get(0).to_string(scope).unwrap().to_rust_string_lossy(scope);
        println!("[JS] {}", message);
    }
}
