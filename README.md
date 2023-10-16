# krush-engine: The engine that powers the Kreative Rust Shell

The krush-engine is a library that has a simple command parser and interpreter that is used for the [krush](https://www.github.com/OJarrisonn/krush) shell interface, but can be used standalone.

# What it does?

Once you've defined an ```Engine``` with all the commands you want using the ```register_command``` method. You can call the method ```evaluate``` with a ```String``` which returns a ```Result```. The first word of this string is the command name, if you've registered a command with that name, the ```Engine``` will parse the rest of the input to get the arguments you've setted. If everything goes right, your callback function will be called.

# TODO
- [ ] List of any Type
- [ ] Support quoted strings in List

# Note

This project were written in a day to be used for my homework so there are several unimplemented features and potential bugs (actually, ```krush``` doesn't even exists at the moment).

# Changelog
## v0.3.0
- Now a Engine can be captured by a thread closure
- Values captured by a definition closure must be Arc<Mutex<T>>
- A defition closure must be a ``move`` closure
## v0.2.0
- Now a Definition uses a closure instead of a function pointer, which allows to modify values in the surrounding context by putting them into ``Cell`` or ``RefCell``(for types who doesn't implement the ``Copy`` trait)
- Added ```definition!``` macro which creates a new ```Definition``` from an array of ```Type``` and a closure.

# Example:
```rust
use std::cell::{Cell, RefCell};

use krush_engine::{Engine, Definition, Type, definition};

fn main() {
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
```
Output: 
```
Hello World!
Last is: Hello World!
This is Krush Engine!
Last is: This is Krush Engine!
Thank You!
Last is: Thank You!
Count is 3
```