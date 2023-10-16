use std::sync::{Mutex, Arc};

use crate::error::Error;

pub type Callback<'a> = Arc<Mutex<dyn FnMut(Vec<Value>) -> Option<String> + 'a + Send>>;

/// Used to describe the type of each argument in a [`Definition`]
#[derive(Clone, Copy)]
pub enum Type {
    Str,
    Int,
    Float,
    Bool,
    List
}

/// The values obtained from parsing a command arguments following its [`Definition`]
pub enum Value {
    Str(String),
    Int(i32),
    Float(f64),
    Bool(bool),
    List(Vec<String>)
}

/// Structure where to set the arguments using [`Type`] and the callback function to be called on the command evaluation by [`Engine::evaluate`] [`crate::Value::unwrap_str()`]
#[derive(Clone)]
pub struct Definition<'a> {
    args: Vec<Type>,
    callback: Callback<'a>
}

impl<'a> Definition<'a> {
    pub fn build(args: &[Type], callback: Callback<'a>) -> Self {
        Self::new(args.to_vec(), callback)
    }
    
    pub fn new(args: Vec<Type>, callback: Callback<'a>) -> Self {
        Self { args, callback }
    }

    pub fn args(&self) -> &Vec<Type> {
        &self.args
    }

    pub fn callback(&mut self) -> &mut Callback<'a> {
        &mut self.callback
    }
}

impl Value {
    pub fn unwrap_str(&self) -> Result<String, Error> {
        match self {
            Value::Str(s) => Ok(s.to_owned()),
            _ => Err(Error("[VALUE UNWRAP] This isn't a string".to_string()))
        }
    }

    pub fn unwrap_i32(&self) -> Result<i32, Error> {
        match self {
            Value::Int(i) => Ok(*i),
            _ => Err(Error("[VALUE UNWRAP] This isn't an i32".to_string()))
        }
    }

    
    pub fn unwrap_f64(&self) -> Result<f64, Error> {
        match self {
            Value::Float(f) => Ok(*f),
            _ => Err(Error("[VALUE UNWRAP] This isn't a f64".to_string()))
        }
    }

    
    pub fn unwrap_bool(&self) -> Result<bool, Error> {
        match self {
            Value::Bool(b) => Ok(*b),
            _ => Err(Error("[VALUE UNWRAP] This isn't a bool".to_string()))
        }
    }

    
    pub fn unwrap_list(&self) -> Result<Vec<String>, Error> {
        match self {
            Value::List(l) => Ok(l.to_owned()),
            _ => Err(Error("[VALUE UNWRAP] This isn't a list".to_string()))
        }
    }
}