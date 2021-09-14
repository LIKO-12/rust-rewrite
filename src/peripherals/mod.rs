pub(crate) mod arithmetic_peripheral;
// Currently broken
//pub(crate) mod simple_peripheral;

use v8::*;

pub trait Peripheral {
    /// Must return the peripheral's type.
    /// example: "Display", "Keyboard", etc...
    fn type_name(&self) -> &'static str;

    /// Creates an ObjectTemplate within the provided scope, which would contain the peripheral's JS api.
    fn js_api<'a>(&self, scope: &mut HandleScope<'a, ()>) -> Local<'a, ObjectTemplate>;
}

pub fn add_method(
		template: Local<ObjectTemplate>,
		scope: &mut HandleScope<'_, ()>,
		name: &str,
		callback: impl MapFnTo<FunctionCallback>) {
	
	template.set(
		String::new(scope, name).unwrap().into(),
		FunctionTemplate::new(scope, callback).into(),
	);
}