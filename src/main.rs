#![allow(dead_code)]
// cargo watch -c -q -s 'cargo rustc -- -Awarnings -Zno-codegen && cargo test'
// https://www.lysator.liu.se/c/ANSI-C-grammar-l.html#comment

use crate::parser::Parser;
use crate::source::{CodeSource, SourceReader};
use crate::tokenizer::{Tokenizer};
use crate::compiler::Compiler;
use crate::runtime::Runtime;
use crate::builtins::register_builtins;

mod source;
mod tokenizer;
mod parser;
mod ast;
mod compiler;
mod run;
mod runtime;
mod builtins;

fn main() {
    let source = CodeSource::file("pruebas.txt");
    let reader = SourceReader::new(source);
    let tokenizer = Tokenizer::new(reader);
    let mut parser = Parser::new(tokenizer);
    let program = parser.parse_program().expect("Unable to parse program");

    let mut compiler = Compiler::new();
    let compiled_program = compiler.compile(program).expect("Unable to compile program");

    println!("{:#?}", compiled_program);


    let mut runtime = Runtime::new();
    register_builtins(&mut runtime);

    let result = runtime.run(compiled_program);

    println!("{:#?}", result);
}
