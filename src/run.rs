use std::collections::HashMap;

use crate::ast::TypeDef;
use crate::runtime::{Runtime, RuntimeError};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct CompiledProgram {
    pub root_function: CompiledFunction,
}

#[derive(Debug, Clone)]
pub struct CompiledFunction {
    pub args: usize,
    pub code: Vec<Inst>,
    pub functions: HashMap<usize, CompiledFunction>,
    pub instance_classes: HashMap<String, InstanceClass>,
}

#[derive(Clone)]
pub struct BuiltinFunction {
    pub args: usize,
    pub func: Box<fn(&mut Runtime, Vec<Value>) -> Result<Value, RuntimeError>>,
}

#[derive(Debug, Clone)]
pub struct InstanceClass {
    pub id: usize,
    pub typedef: Rc<TypeDef>,
    pub variant: String,
    pub properties: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum Inst {
    Set(String),
    Int(i32),
    Float(f32),
    String(String),
    Call(String),
    List(usize),
    Tuple(usize),
    Function(usize),
    Return,
}

#[derive(Debug, Clone)]
pub enum Type {
    Int,
    Float,
    String,
    List { items: Box<Type> },
    Tuple { values: Vec<Type> },
    Function { func: usize },
    Instance { class: usize },
}

#[derive(Debug, Clone)]
pub enum Value {
    Unit,
    Int(i32),
    Float(f32),
    String(String),
    List(Vec<Value>),
    Tuple(Vec<Value>),
    Function { func: usize },
    Instance(Instance),
}

#[derive(Debug, Clone)]
pub struct Instance {
    pub class: usize,
    pub properties: Vec<Value>,
}