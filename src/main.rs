mod cli;

use crate::cli::cli_parser::{Commands, CpastCli};
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use cpast::{compile_and_test, generator};

fn main() {
    let cli_instance = CpastCli::new();
    if let Some(command) = cli_instance.command {
        match command {
            Commands::Test(args) => {
                let correct_binding = args.correct_file.unwrap_or(String::from(""));
                let test_binding = args.test_file.unwrap_or(String::from(""));
                let language = args.generator.unwrap_or(String::from(""));
                let iterations = args.iterations;

                compile_and_test(correct_binding, test_binding, language, iterations);
            }
            Commands::Generate(args) => {
                if args.generator.is_none() {
                    println!("[GENERATOR] Generator language is required!");
                } else {
                    let language = args.generator.unwrap_or(String::from(""));
                    let generated_tescases = generator(language);
                    println!("Generated Testcase");
                    println!("=====================================");
                    println!("{}", &generated_tescases);
                    println!("=====================================");

                    if args.clipboard {
                        let mut ctx = ClipboardContext::new().unwrap();
                        ctx.set_contents(generated_tescases.to_owned()).unwrap();

                        // get_contents is required for set_contents to work
                        // Refer https://github.com/aweinstock314/rust-clipboard/issues/86
                        let _ = ctx.get_contents();
                        println!("Copied to clipboard successfully!");
                    }
                }
            }
        }
    } else {
        println!("Invalid Usage! Use cpast --help for more info");
    }
}
