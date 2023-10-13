use std::collections::HashMap;

use crate::{error::Error, parser};

use self::command::Definition;

pub mod command;

pub struct Engine {
    registry: HashMap<String, Definition>
}

impl Engine {
    pub fn new() -> Self {
        Self { registry: HashMap::new() }
    }

    pub fn register_command(&mut self, name: &'static str, definition: Definition) {
        self.registry.insert(name.to_string(), definition);
    }

    pub fn evaluate(&mut self, command: String) -> Result<Option<String>, Error> {
        let mut command = command;
        let name = &parser::get_name(&mut command);

        
        match self.registry.get_mut(name) {
            Some(definition) => {
                let args = parser::parse_args(definition.args(), &mut command)?;

                Ok((definition.callback())(args))
            },
            None => Err(Error(format!("Unknown command: {}", name))),
        }
    }
}