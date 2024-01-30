mod cli;

use crate::cli::cli_parser::{Commands, CpastCli};
use colored::Colorize;
use cpast::{compile_and_test, generator};
use std::process::exit;

#[cfg(feature = "clipboard")]
use cli_clipboard::{ClipboardContext, ClipboardProvider};

#[tokio::main(flavor = "multi_thread", worker_threads = 16)]
async fn main() {
    let cli_instance = CpastCli::new();
    if let Some(command) = cli_instance.command {
        match command {
            Commands::Test(args) => {
                let correct_binding = args.correct_file.unwrap_or_else(String::new);
                let test_binding = args.test_file.unwrap_or_else(String::new);
                let language = args.generator.unwrap_or_else(String::new);
                let iterations = args.iterations;
                let no_stop = args.no_stop;
                let do_force_compile = args.force_recompile;

                compile_and_test(
                    correct_binding,
                    test_binding,
                    language,
                    iterations,
                    no_stop,
                    do_force_compile,
                )
                .await
                .unwrap_or_else(|err| err.print_and_exit());
            }
            Commands::Generate(args) => {
                if args.generator.is_none() {
                    println!("{}", "[GENERATOR] Generator language is required!".red());
                } else {
                    let language = args.generator.unwrap_or_else(String::new);
                    let generated_testcases = generator(language).unwrap_or_else(|err| {
                        eprintln!("{:#?} {}", err, err.get_msg());
                        exit(1);
                    });
                    println!("{}", "Generated Testcase".green());
                    println!("=====================================");
                    println!("{}", &generated_testcases);
                    println!("=====================================");
                    if args.clipboard {
                        #[cfg(all(
                            any(target_os = "windows", target_os = "linux", target_os = "macos"),
                            feature = "clipboard"
                        ))]
                        {
                            let mut ctx = ClipboardContext::new().unwrap();
                            ctx.set_contents(generated_testcases).unwrap();

                            // get_contents is required for set_contents to work
                            // Refer https://github.com/aweinstock314/rust-clipboard/issues/86
                            let _ = ctx.get_contents();
                            println!("{}", "Copied to clipboard successfully!".green());
                        }

                        #[cfg(any(
                            not(any(
                                target_os = "windows",
                                target_os = "linux",
                                target_os = "macos"
                            )),
                            not(feature = "clipboard")
                        ))]
                        println!(
                            "{}",
                            "Clipboard Features not enabled during compilation/device not supported!"
                                .yellow()
                        );
                    }
                }
            }
        }
    } else {
        println!("{}", "Invalid Usage! Use cpast --help for more info".red());
    }
}
