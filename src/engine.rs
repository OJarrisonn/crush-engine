use std::collections::HashMap;

use crate::{error::Error, parser};

use self::command::Definition;

pub mod command;

/// The place where to register your commands and run them
pub struct Engine {
    registry: HashMap<String, Definition>
}

impl Engine {
    /// Creates a new empty `Engine`
    pub fn new() -> Self {
        Self { registry: HashMap::new() }
    }

    /// Adds a new command to the `Engine` registry with given name and [`Definition`]
    pub fn register_command(&mut self, name: &'static str, definition: Definition) {
        self.registry.insert(name.to_string(), definition);
    }

    /// Receives a command and call the respective callback if the command was registered in [`Definition::register_command`]
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