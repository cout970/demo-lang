use std::collections::HashMap;
use std::rc::Rc;

use crate::ast::TypeDef;
use crate::run::{BuiltinFunction, CompiledFunction, CompiledProgram, Inst, Instance, InstanceClass, Value};

#[derive(Debug, Clone)]
pub enum RuntimeError {
    StackUnderflow,
    UndefinedName(String),
    Custom(String),
}

pub struct Runtime {
    builtin_functions: HashMap<String, BuiltinFunction>,
    builtin_instance_classes: HashMap<String, Rc<InstanceClass>>,
    builtin_id_to_class: HashMap<usize, Rc<InstanceClass>>,
    next_id: usize,
}

struct Env {
    frames: Vec<StackFrame>,
}

struct StackFrame {
    variables: HashMap<String, Value>,
    functions: HashMap<usize, CompiledFunction>,
    instance_classes: HashMap<String, Rc<InstanceClass>>,
    id_to_class: HashMap<usize, Rc<InstanceClass>>,
}

impl Runtime {
    pub fn new() -> Self {
        Runtime {
            builtin_functions: Default::default(),
            builtin_instance_classes: Default::default(),
            builtin_id_to_class: Default::default(),
            next_id: 100_000
        }
    }

    pub fn run(&mut self, cp: CompiledProgram) -> Result<Value, RuntimeError> {
        let mut env = Env::new();

        env.push(&cp.root_function);
        let value = self.run_function(&mut env, &cp.root_function, vec![])?;
        env.pop();

        Ok(value)
    }

    pub fn register_func(&mut self, name: &str, args: usize, func: fn(&mut Runtime, Vec<Value>) -> Result<Value, RuntimeError>) {
        self.builtin_functions.insert(name.to_string(), BuiltinFunction {
            args,
            func: Box::new(func),
        });
    }

    pub fn register_type(&mut self, def: TypeDef) {
        let def = Rc::new(def);

        for variant in &def.variants {
            let class = InstanceClass {
                id: self.next_id,
                typedef: def.clone(),
                variant: variant.name.to_string(),
                properties: variant.properties.clone(),
            };
            self.next_id += 1;

            let rc = Rc::new(class);

            self.builtin_id_to_class.insert(rc.id, rc.clone());
            self.builtin_instance_classes.insert(variant.name.to_string(), rc);
        }
    }

    fn run_function(&mut self, env: &mut Env, p: &CompiledFunction, args: Vec<Value>) -> Result<Value, RuntimeError> {
        let mut ip = 0;
        let mut stack = args;

        while ip < p.code.len() {
            let inst = &p.code[ip];
            ip += 1;

            match inst {
                Inst::Set(name) => {
                    env.set(name, stack.pop().ok_or_else(|| RuntimeError::StackUnderflow)?);
                }
                Inst::Int(value) => {
                    stack.push(Value::Int(*value));
                }
                Inst::Float(value) => {
                    stack.push(Value::Float(*value));
                }
                Inst::String(value) => {
                    stack.push(Value::String(value.clone()));
                }
                Inst::Call(name) => {
                    // Variable
                    if let Some(value) = env.get(name) {
                        if let Value::Function { func } = &value {
                            let func = env.get_function(*func).unwrap();
                            let mut args = vec![];

                            for _ in 0..func.args {
                                let val = stack.pop().ok_or_else(|| RuntimeError::StackUnderflow)?;
                                args.push(val);
                            }

                            env.push(&func);
                            let result = self.run_function(env, &func, args)?;
                            env.pop();

                            stack.push(result);
                        } else {
                            stack.push(value);
                        }
                        continue;
                    }

                    // TypeDef
                    if let Some(instance_class) = env.get_instance_class(name) {
                        let mut properties = vec![];

                        for _ in 0..instance_class.properties.len() {
                            let val = stack.pop().ok_or_else(|| RuntimeError::StackUnderflow)?;
                            properties.push(val);
                        }

                        let value = Value::Instance(Instance { class: instance_class.id, properties });
                        stack.push(value);
                        continue;
                    }

                    // Builtin function
                    if let Some(func) = self.builtin_functions.get(name) {
                        let mut args = vec![];

                        for _ in 0..func.args {
                            let val = stack.pop().ok_or_else(|| RuntimeError::StackUnderflow)?;
                            args.push(val);
                        }

                        let result = (func.func.clone())(self, args)?;

                        stack.push(result);
                        continue;
                    }

                    // Builtin TypeDef
                    if let Some(instance_class) = self.builtin_instance_classes.get(name) {
                        let mut properties = vec![];

                        for _ in 0..instance_class.properties.len() {
                            let val = stack.pop().ok_or_else(|| RuntimeError::StackUnderflow)?;
                            properties.push(val);
                        }

                        let value = Value::Instance(Instance { class: instance_class.id, properties });
                        stack.push(value);
                        continue;
                    }

                    // Error not found
                    return Err(RuntimeError::UndefinedName(name.to_string()));
                }
                Inst::List(items) => {
                    let mut values = vec![];

                    // TODO check everything has the same type
                    for _ in 0..*items {
                        values.push(stack.pop().ok_or_else(|| RuntimeError::StackUnderflow)?);
                    }

                    stack.push(Value::List(values));
                }
                Inst::Tuple(items) => {
                    let mut values = vec![];

                    for _ in 0..*items {
                        values.push(stack.pop().ok_or_else(|| RuntimeError::StackUnderflow)?);
                    }

                    stack.push(Value::Tuple(values));
                }
                Inst::Function(func) => {
                    stack.push(Value::Function { func: *func });
                }
                Inst::Return => {
                    return Ok(stack.pop().ok_or_else(|| RuntimeError::StackUnderflow)?);
                }
            }
        }

        Ok(stack.pop().unwrap_or(Value::Unit))
    }
}

impl Env {
    fn new() -> Self {
        Env {
            frames: vec![]
        }
    }

    fn get(&self, name: &str) -> Option<Value> {
        for frame in self.frames.iter().rev() {
            if let Some(val) = frame.variables.get(name) {
                return Some(val.clone());
            }
        }

        None
    }

    fn set(&mut self, name: &str, value: Value) {
        let frame = self.frames.iter_mut().rev().next();
        if let Some(frame) = frame {
            frame.variables.insert(name.to_string(), value);
        }
    }

    fn get_instance_class(&self, name: &str) -> Option<Rc<InstanceClass>> {
        for frame in self.frames.iter().rev() {
            if let Some(val) = frame.instance_classes.get(name) {
                return Some(val.clone());
            }
        }

        None
    }

    fn get_function(&self, id: usize) -> Option<CompiledFunction> {
        for frame in self.frames.iter().rev() {
            if let Some(val) = frame.functions.get(&id) {
                return Some(val.clone());
            }
        }

        None
    }

    fn push(&mut self, func: &CompiledFunction) {
        let mut id_to_class: HashMap<usize, Rc<InstanceClass>> = HashMap::new();
        let mut instance_classes: HashMap<String, Rc<InstanceClass>> = HashMap::new();

        for class in func.instance_classes.values() {
            let rc = Rc::new(class.clone());

            id_to_class.insert(class.id, rc.clone());
            instance_classes.insert(class.variant.to_string(), rc);
        }

        self.frames.push(StackFrame {
            variables: Default::default(),
            functions: func.functions.clone(),
            id_to_class,
            instance_classes,
        });
    }

    fn pop(&mut self) {
        self.frames.pop().unwrap();
    }
}