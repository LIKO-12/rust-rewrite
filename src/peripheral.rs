use *;

pub trait Peripheral {
    /// Must return the peripheral's type.
    /// example: "Display", "Keyboard", etc...
    fn type_name(&self) -> &'static str;

    /// Creates an ObjectTemplate within the provided scope, which would contain the peripheral's JS api.
    fn js_api<'a>(&self, scope: &mut HandleScope<'a, ()>) -> Local<'a, ObjectTemplate>;
}

pub fn add_method(template: ObjectTemplate, scope: HandleScope, name: impl ToString, callback: impl MapFnTo<FunctionCallback>) {
	template.set(
		String::new(scope, name).unwrap().into(),
		FunctionTemplate::new(scope, callback).into(),
	);
}