use crate::ast::{Expression, Operator, Program, Statement, UnaryOperator};
use crate::run::{CompiledFunction, CompiledProgram, Inst, InstanceClass};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum CompileError {}

pub struct Compiler {
    next_id: usize
}

impl Compiler {
    pub fn new() -> Self {
        Compiler { next_id: 0 }
    }

    pub fn compile(&mut self, program: Program) -> Result<CompiledProgram, CompileError> {
        let mut root = CompiledFunction {
            args: 0,
            code: vec![],
            functions: Default::default(),
            instance_classes: Default::default(),
        };

        for stm in program.statements {
            self.compile_statement(&mut root, stm)?;
        }

        Ok(CompiledProgram {
            root_function: root,
        })
    }

    fn compile_statement(&mut self, node: &mut CompiledFunction, stm: Statement) -> Result<(), CompileError> {
        match stm {
            Statement::Variable(var) => {
                self.compile_expression(node, var.value)?;
                node.code.push(Inst::Set(var.name));
            }
            Statement::Expression(e) => {
                self.compile_expression(node, e)?;
            }
            Statement::TypeDef(def) => {
                let def = Rc::new(def);

                for variant in &def.variants {
                    let class = InstanceClass {
                        id: self.next_id(),
                        typedef: def.clone(),
                        variant: variant.name.to_string(),
                        properties: variant.properties.clone(),
                    };

                    node.instance_classes.insert(variant.name.to_string(), class);
                }
            }
        }

        Ok(())
    }

    fn compile_expression(&mut self, node: &mut CompiledFunction, expr: Expression) -> Result<(), CompileError> {
        match expr {
            Expression::UnaryOperator { operator, expr } => {
                self.compile_expression(node, *expr)?;
                let op = match operator {
                    UnaryOperator::Plus => "unary_plus",
                    UnaryOperator::Minus => "unary_minus",
                    UnaryOperator::Not => "unary_not",
                };
                node.code.push(Inst::Call(op.to_string()));
            }
            Expression::Int { value } => {
                node.code.push(Inst::Int(value));
            }
            Expression::Float { value } => {
                node.code.push(Inst::Float(value));
            }
            Expression::String { value } => {
                node.code.push(Inst::String(value));
            }
            Expression::FunCall { name, args } => {
                for expr in args {
                    self.compile_expression(node, expr)?;
                }
                node.code.push(Inst::Call(name));
            }
            Expression::Operator { operator, left, right } => {
                self.compile_expression(node, *left)?;
                self.compile_expression(node, *right)?;
                let name = match operator {
                    Operator::BiteAnd => "&",
                    Operator::BiteOr => "|",
                    Operator::Plus => "+",
                    Operator::Minus => "-",
                    Operator::Times => "*",
                    Operator::Div => "/",
                    Operator::Rem => "%",
                    Operator::Less => "<",
                    Operator::Greater => ">",
                    Operator::LessEquals => "<=",
                    Operator::GreaterEquals => ">=",
                    Operator::And => "&&",
                    Operator::Or => "||",
                    Operator::Xor => "^",
                    Operator::Equals => "==",
                    Operator::NotEquals => "!=",
                };
                node.code.push(Inst::Call(name.to_string()));
            }
            Expression::List { items } => {
                let len = items.len();
                for expr in items {
                    self.compile_expression(node, expr)?;
                }
                node.code.push(Inst::List(len));
            }
            Expression::Tuple { values } => {
                let len = values.len();
                for expr in values {
                    self.compile_expression(node, expr)?;
                }
                node.code.push(Inst::Tuple(len));
            }
            Expression::Lambda { args, code } => {
                let mut lambda = CompiledFunction {
                    args: args.len(),
                    code: vec![],
                    functions: Default::default(),
                    instance_classes: Default::default(),
                };

                for arg in args.into_iter().rev() {
                    lambda.code.push(Inst::Set(arg));
                }

                for stm in code {
                    self.compile_statement(&mut lambda, stm)?;
                }

                let id = self.next_id();
                node.functions.insert(id, lambda);
                node.code.push(Inst::Function(id));
            }
            Expression::Return { value } => {
                self.compile_expression(node, *value)?;
                node.code.push(Inst::Return);
            }
        }

        Ok(())
    }

    fn next_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}

