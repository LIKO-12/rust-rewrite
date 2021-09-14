use v8::*;

use super::{Peripheral, add_method};

pub struct ArithmeticPeripheral {}

impl ArithmeticPeripheral {
    pub fn new() -> Self {
        Self {}
    }

    fn sum_callback(scope: &mut HandleScope, args: FunctionCallbackArguments, mut rv: ReturnValue) {
        let mut sum = 0.0;

        for i in 0..args.length() {
            let arg = args.get(i);
            let value = arg.number_value(scope).unwrap();
            sum += value;
        }

        rv.set(Number::new(scope, sum).into());
    }
}

impl Peripheral for ArithmeticPeripheral {
    fn type_name(&self) -> &'static str {
        "arithmetic"
    }

    fn js_api<'a>(&self, scope: &mut HandleScope<'a, ()>) -> Local<'a, ObjectTemplate> {
        let api = ObjectTemplate::new(scope);

        add_method(api, scope, "sum", Self::sum_callback);

        api
    }
}