mod cli;

use crate::cli::cli_parser::CliArgs;
use cpast::compile_and_test;

fn main() {
    let args = CliArgs::new();

    let correct_binding = args.correct_file.unwrap_or(String::from(""));
    let test_binding = args.test_file.unwrap_or(String::from(""));
    let language = args.generator.unwrap_or(String::from(""));
    let iterations = args.iterations;

    compile_and_test(correct_binding, test_binding, language, iterations);
}
