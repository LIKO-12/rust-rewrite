// TODO: Use a shared function instead of a macro, to avoid increasing the binary's size.

#[macro_export]
macro_rules! add_method {
    ($template:expr, $scope: expr, $name:literal, $callback:expr) => {
        $template.set(
            String::new($scope, $name).unwrap().into(),
            FunctionTemplate::new($scope, $callback).into(),
        );
    };
}
