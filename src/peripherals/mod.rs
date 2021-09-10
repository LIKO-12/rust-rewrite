pub mod macros;
pub(crate) mod arithmetic_peripheral;
// Currently broken
//pub(crate) mod simple_peripheral;
pub mod store;

pub trait Peripheral {
    /// Must return the peripheral's type.
    /// example: "Display", "Keyboard", etc...
    fn type_name(&self) -> &'static str;

    /// Creates an ObjectTemplate within the provided scope, which would contain the peripheral's JS api.
    fn js_api<'a>(&self, scope: &mut v8::HandleScope<'a, ()>) -> v8::Local<'a, v8::ObjectTemplate>;
}
