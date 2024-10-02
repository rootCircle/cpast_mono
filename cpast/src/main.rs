mod cli;
mod cmd;

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
            }
        } else {
            println!("{}", "Invalid Usage! Use cpast --help for more info".red());
        }
    }
}
