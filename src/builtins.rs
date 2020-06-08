use crate::ast::{TypeDef, TypeDefVariant};
use crate::run::Value;
use crate::runtime::{Runtime, RuntimeError};

pub fn register_builtins(runtime: &mut Runtime) {
    runtime.register_func("print", 1, |_, args| {
        let param = args.into_iter().next().unwrap();

        println!("{:?}", param);

        Ok(param)
    });

    runtime.register_func("unary_minus", 1, |_, args| {
        let param = args.into_iter().next().unwrap();
        match &param {
            Value::Int(value) => Ok(Value::Int(-(*value))),
            Value::Float(value) => Ok(Value::Float(-(*value))),
            _ => Err(RuntimeError::Custom(format!("Unable to negate non numeric value: {:?}", param)))
        }
    });

    runtime.register_func("unary_plus", 1, |_, args| {
        let param = args.into_iter().next().unwrap();
        match &param {
            Value::Int(value) => Ok(Value::Int(*value)),
            Value::Float(value) => Ok(Value::Float(*value)),
            _ => Err(RuntimeError::Custom(format!("Unable to use unary plus on non numeric value: {:?}", param)))
        }
    });

    // runtime.register_func("unary_not", 1, |run, args| {
    //     let param = args.into_iter().next().unwrap();
    //     let inst: Instance = param.try_into()?;
    //
    // });


    runtime.register_type(TypeDef {
        name: "Boolean".to_string(),
        variants: vec![
            TypeDefVariant { name: "True".to_string(), properties: vec![] },
            TypeDefVariant { name: "False".to_string(), properties: vec![] },
        ],
    });
}