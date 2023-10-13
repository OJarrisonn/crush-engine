use crush_engine::{Engine, Definition, Type};



#[test]
fn engine() {
    let mut e = Engine::new();

    e.register_command("test", Definition::new(vec![Type::Str, Type::Int], |args| {
        let text = args[0].unwrap_str().unwrap();
        let num = args[1].unwrap_i32().unwrap();

        Some(format!("String is: {text}. Integer is {num}"))
    }));

    e.register_command("lines", Definition::new(vec![Type::List], |args| {
        let list = args[0].unwrap_list().unwrap();
        let mut res = String::new();

        for e in list {
            res.push_str(&e);
            res.push_str("\n");
        }

        Some(res)
    }));

    let command = "lines minha mensagem muito maneira mesmo slk kkkkkkkk".to_string();
    println!("{}", e.evaluate(command).unwrap().unwrap());
}