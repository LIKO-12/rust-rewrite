use std::collections::HashMap;
use super::{Peripheral, MethodsMap};

struct SimplePeripheral {}

impl SimplePeripheral {
    fn new() -> Self {
        SimplePeripheral {}
    }

    fn sum(scope: &mut v8::HandleScope, args: v8::FunctionCallbackArguments, mut result: v8::ReturnValue) {
        let mut sum = 0.0;

        for i in 0..args.length() {
            let arg = args.get(i);
            let value = arg.number_value(scope).unwrap();
            sum += value;
        }

        result.set(v8::Number::new(scope, sum).into());
    }
}

impl Peripheral for SimplePeripheral {
    fn name() -> &'static str {
        "simple_peripheral"
    }

    fn methods() -> MethodsMap {
        let mut methods: MethodsMap = HashMap::new();

        methods.insert("sum", Self::sum);

        methods
    }
}