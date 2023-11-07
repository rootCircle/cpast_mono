#![allow(unused)]

mod language;
mod program_store;
mod utils;
mod clex_language;

use std::path::Path;
use std::process::exit;

use crate::program_store::ProgramStore;
use crate::clex_language::{lexer, parser, generator};

pub fn compile_and_test(src_binding:String, test_binding: String, language: String, iterations: usize) {

    let store = ProgramStore::new(
        Path::new(&src_binding),
        Path::new(&test_binding)
    );

    let mut token = lexer::Tokens::new(language);
    token.scan_tokens();

    let mut parser = parser::Parser::new_from_tokens(token);
    parser.parser();

    let mut gen = generator::Generator::new(parser);

    for iter in 1..=iterations {
        gen.traverse_ast();

        match store.run_code(&gen.output_text) {
            Ok((true, _, _)) => println!("Testcase {iter} ran successfully!"),
            Ok((false, actual, expected)) => {
                println!("Testcase {iter} failed!");
                println!("INPUT\n{}", &gen.output_text);
                println!("==============================");
                println!("EXPECTED OUTPUT\n{}", expected);
                println!("==============================");
                println!("ACTUAL OUTPUT\n{}", actual);
                exit(1);
            },
            Err(err) => println!("Error matching the file! {}", err)
        }


        gen.reset_output();
    }
}
