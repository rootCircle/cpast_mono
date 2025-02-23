mod cli;
mod cmd;
mod error_types;

use crate::cli::cli_parser::{CpastCommand, CpastSubcommands};
use colored::Colorize;

#[tokio::main(flavor = "multi_thread", worker_threads = 16)]
async fn main() {
    let cli_instance_wrap = CpastCommand::new();
    if let Some(cli_instance) = cli_instance_wrap {
        if let Some(command) = cli_instance.subcommand {
            match command {
                CpastSubcommands::Test(args) => {
                    cmd::test::test_call(args).await;
                }
                CpastSubcommands::Generate(args) => {
                    cmd::generate::generate_call(args);
                }
                CpastSubcommands::Ai(args) => {
                    cmd::ai::generate_clex_from_input_format_and_constraints(args)
                        .await
                        .unwrap_or_else(|err| {
                            eprintln!("{}", err);
                        });
                }
            }
        } else {
            println!("{}", "Invalid Usage! Use cpast --help for more info".red());
        }
    }
}
