#![allow(unused)]

mod cli;
mod language;
mod program_store;
mod utils;

use std::path::Path;

use crate::cli::cli_parser::CliArgs;
use crate::program_store::ProgramStore;
use crate::utils::file_utils::read_file;
use crate::language::Language;

fn main() {
    let args = CliArgs::new();

    println!(
        "{:#?}",
        ProgramStore::new(
            Path::new(&args.source_file.unwrap_or(String::from(""))),
            Path::new(&args.test_file.unwrap_or(String::from("")))
        )
    );

    // println!("{}", read_file(Path::new("src/main.rs")).unwrap());

    // println!("{:#?}", Language::compile_language(Path::new("./1.c"), &Language::C));

    println!("{:#?}", Language::run_program_code(Path::new("./1.c"), "Helloajkhdkjs"));
}
