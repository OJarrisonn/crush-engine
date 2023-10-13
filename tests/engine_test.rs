use krush_engine::{Engine, Definition, Type, Value};

#[test]
fn create() {
    let mut last = Box::new(0);
    let mut engine = Engine::new();

    fn command_func(args: Vec<Value>, last: Box<i32>) -> Option<String> {
        *last += 1;
        println!("{}", last);
        None
    }

    engine.register_command("print", Definition::new(Vec::from([Type::Int]), Box::new(|args| command_func(args, last))));
}