mod cli;

use crate::cli::cli_parser::CliArgs;
use cpast::compile_and_test;

fn main() {
    let args = CliArgs::new();

    let src_binding = args.source_file.unwrap_or(String::from(""));
    let test_binding = args.test_file.unwrap_or(String::from(""));
    let language = args.generator.unwrap_or(String::from(""));
    let iterations = args.iterations;

    compile_and_test(src_binding, test_binding, language, iterations);
}
