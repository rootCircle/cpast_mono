mod cli;
mod cmd;

use crate::cli::cli_parser::{Commands, CpastCli};
use colored::Colorize;

#[tokio::main(flavor = "multi_thread", worker_threads = 16)]
async fn main() {
    let cli_instance_wrap = CpastCli::new();
    if let Some(cli_instance) = cli_instance_wrap {
        if let Some(command) = cli_instance.command {
            match command {
                Commands::Test(args) => {
                    cmd::test::test_call(args).await;
                }
                Commands::Generate(args) => {
                    cmd::generate::generate_call(args);
                }
            }
        } else {
            println!("{}", "Invalid Usage! Use cpast --help for more info".red());
        }
    }
}
