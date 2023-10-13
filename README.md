# krush-engine: The engine that powers the Kreative Rust Shell

The krush-engine is a library that has a simple command parser and interpreter that is used for the [krush](https://www.github.com/OJarrisonn/krush) shell interface, but can be used standalone.

# Note

This project were written in a day to be used for my homework so there are several unimplemented features and potential bugs (actually, ```krush``` doesn't even exists at the moment).

# Example:
```rust
use krush_engine::{Engine, Type};

fn main() {
    let mut engine = Engine::new();

    engine.register_command("print", Definition::new(vec![Type::Str], |args| { 
        let text = args[0].unwrap_str().unwrap();
        println!("{}", text);

        None 
    }));

    let cmd = "print \"Hello World!\"";
    engine.evaluate(cmd);
}
```