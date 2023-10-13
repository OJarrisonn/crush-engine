use std::cell::Cell;

use krush_engine::{Engine, Definition, Type, definition};

#[test]
fn create() {
    let last = Cell::new(0);
    let mut engine = Engine::new();

    engine.register_command("print", definition!([Type::Int], |args| {
        let res = args[0].unwrap_i32().unwrap();
        last.set(res);
        println!("{}", last.get());
        None
    }));

    let _ = engine.evaluate("print 5".to_string());
    let _ = engine.evaluate("print 8".to_string());

    println!("{}", last.get());
}