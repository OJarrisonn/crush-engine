use crate::{Value, Type, error::Error};

/// Takes a command String, returns the name, and strip it from the command String
pub fn get_name(command: &mut String) -> String {
    let pos = command.find(' ').unwrap_or_else(|| command.len());
    let name = (&command[..pos]).to_string();
    command.drain(..pos+1);
    name
}

/// Receives a vector of the arg types and parse a string of args, returning a Vec of values where each value matches the corresponding Type, or an Error if it fails
pub fn parse_args(definition: &Vec<Type>, raw_args: &mut String) -> Result<Vec<Value>, Error> {
    let mut args: Vec<Value> = vec![];
    let mut raw_args = raw_args.trim();


    for arg in definition {
        if raw_args.len() == 0 {
            return Err(Error("[ARG PARSE] Not enough arguments".to_string()))
        }
        args.push(match arg {
            Type::Str => {
                let pos = if raw_args.chars().next().unwrap() == '"' {
                    raw_args = &raw_args[1..];
                    match raw_args.find('"') {
                        Some(len) => len,
                        None => return Err(Error("[ARG PARSE] There's a quotation mark with no pair".to_string())),
                    }
                } else {
                    raw_args.find(' ').unwrap_or_else(|| raw_args.len())
                };

                let text = &raw_args[..pos];
                raw_args = if pos+1 < raw_args.len() {
                    &raw_args[pos+1..].trim_start()
                } else {
                    &raw_args[raw_args.len()..]
                };

                Value::Str(text.to_string())
            },
            Type::Int => {
                let pos = raw_args.find(' ').unwrap_or_else(|| raw_args.len());
                
                let num = &raw_args[..pos];
                raw_args = if pos+1 < raw_args.len() {
                    &raw_args[pos+1..].trim_start()
                } else {
                    &raw_args[raw_args.len()..]
                };

                let num: i32 = match num.parse() {
                    Ok(n) => n,
                    Err(_) => return Err(Error(format!("Expected an integer, got {}.", num))),
                };

                Value::Int(num)
            },
            Type::Float => {
                let pos = raw_args.find(' ').unwrap_or_else(|| raw_args.len());
                
                let num = &raw_args[..pos];
                raw_args = if pos+1 < raw_args.len() {
                    &raw_args[pos+1..].trim_start()
                } else {
                    &raw_args[raw_args.len()..]
                };

                let num: f64 = match num.parse() {
                    Ok(n) => n,
                    Err(_) => return Err(Error(format!("Expected a float, got {}.", num))),
                };

                Value::Float(num)
            },
            Type::Bool => {
                let pos = raw_args.find(' ').unwrap_or_else(|| raw_args.len());
                
                let boolean = &raw_args[..pos];
                raw_args = if pos+1 < raw_args.len() {
                    &raw_args[pos+1..].trim_start()
                } else {
                    &raw_args[raw_args.len()..]
                };

                let boolean: bool = match boolean.parse() {
                    Ok(b) => b,
                    Err(_) => return Err(Error(format!("Expected a number, got {}.", boolean))),
                };

                Value::Bool(boolean)
            },
            Type::List => {
                let splited_args = raw_args.split(' ');
                let mut list = Vec::new();

                for e in splited_args {
                    list.push(e.to_string());
                }

                raw_args = &raw_args[raw_args.len()..];
                Value::List(list)
            },
        })
    }

    Ok(args)
}