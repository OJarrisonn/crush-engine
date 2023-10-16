#[macro_export]
macro_rules! definition {
    ($args:expr, $callback:expr) => {
        Definition::new($args.to_vec(), Arc::new(Mutex::new($callback)))
    };
}