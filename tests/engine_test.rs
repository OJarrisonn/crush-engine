use std::cell::{Cell, RefCell};

use krush_engine::{Engine, Definition, Type, definition};

#[test]
fn create() {
    let last = RefCell::new(String::new());
    let count = Cell::new(0);
    let mut engine = Engine::new();

    engine.register_command("print", definition!([Type::Str], |args| { 
        let text = args[0].unwrap_str().unwrap();
        println!("{}", text);

        *last.borrow_mut() = text;
        count.set(count.get() + 1);

        None 
    }));

    let _ = engine.evaluate("print \"Hello World!\"".to_string());
    println!("Last is: {}", last.borrow());
    let _ = engine.evaluate("print \"This is Krush Engine!\"".to_string());
    println!("Last is: {}", last.borrow());
    let _ = engine.evaluate("print \"Thank You!\"".to_string());
    println!("Last is: {}\nCount is {}", last.borrow(), count.get());
}