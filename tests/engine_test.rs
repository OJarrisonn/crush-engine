use std::{thread, sync::{Arc, Mutex}};

use krush_engine::{Engine, Definition, Type, definition, Value};

#[test]
fn create() {
    let mut engine = Engine::new();
    let text = Arc::new(Mutex::new(String::new()));
    let text_c = Arc::clone(&text);

    engine.register_command("print", definition!([Type::Str], move |args: Vec<Value>| -> Option<String> {
        let msg = args[0].unwrap_str().unwrap();

        let mut r = text_c.lock().unwrap();
        *r = msg;

        println!("{}", &r);
        None
    }));

    let mut engine_thread = engine.clone();
    let mut engine_thread_2 = engine.clone();
    let t1 = thread::spawn(move || engine_thread.evaluate("print \"Mensagem de teste 2\"".to_string()));
    let t2 = thread::spawn(move || engine_thread_2.evaluate("print \"Mensagem de teste 2\"".to_string()));
    let _ = t1.join().unwrap();
    let _ = t2.join().unwrap();
    println!("A mensagem foi: {}", &text.lock().unwrap());
}