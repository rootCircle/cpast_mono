#![allow(unused)]

mod cli;
mod language;
mod program_store;
mod utils;
mod test_language;

use std::path::Path;

use crate::cli::cli_parser::CliArgs;
use crate::program_store::ProgramStore;
// use crate::utils::file_utils::read_file;
// use crate::language::Language;
use crate::test_language::lexer;

fn main() {
    let args = CliArgs::new();

    let src_binding = args.source_file.unwrap_or(String::from(""));
    let test_binding = args.test_file.unwrap_or(String::from(""));

    let store = ProgramStore::new(
        Path::new(&src_binding),
        Path::new(&test_binding)
    );

    // println!("{}", read_file(Path::new("src/main.rs")).unwrap());

    // println!("{:#?}", Language::compile_language(Path::new("./1.c"), &Language::C));

    // println!("{:#?}", Language::run_program_code(Path::new("./1.c"), "Helloajkhdkjs"));

    // println!("{:#?}", store.run_code("Hello"));

    let mut token = lexer::Tokens::new(String::from("N(?:\\\t\nF){[]} SC,"));
    token.scan_tokens();

    println!("{:#?}", token.tokens);
}
