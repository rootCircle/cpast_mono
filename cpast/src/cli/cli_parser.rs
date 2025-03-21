use std::io;

use clap::{Command, CommandFactory, Parser, Subcommand, ValueHint};
use clap_complete::{Generator, Shell, generate};
use colored::Colorize;

const DEFAULT_ITERATIONS_COUNT: usize = 5;

#[derive(Parser)] // requires `derive` feature
#[command(name = "cpast", version, author, about, long_about = None)]
#[command(bin_name = "cpast")]
pub(crate) struct CpastCommand {
    /// Generate Shell Completions
    #[arg(long = "completions", value_enum)]
    completions: Option<Shell>,
    #[command(subcommand)]
    pub(crate) subcommand: Option<CpastSubcommands>,
}

#[derive(Subcommand)] // requires `derive` feature
pub(crate) enum CpastSubcommands {
    /// Compare two files to find the missing edge case
    #[command(author)]
    Test(TestArgs),

    /// Just generate the testcase
    #[command(author)]
    Generate(GenerateArgs),

    /// Generate clex using AI from input format and constraints
    Ai(AiArgs),
}

fn print_completions<G: Generator>(generator: G, cmd: &mut Command) {
    generate(
        generator,
        cmd,
        cmd.get_name().to_string(),
        &mut io::stdout(),
    );
}

#[derive(clap::Args)]
pub(crate) struct TestArgs {
    /// The correct file
    #[arg(short, long, value_hint = ValueHint::FilePath)]
    pub(crate) correct_file: Option<String>,

    /// The test file
    #[arg(short, long, required = true, value_hint = ValueHint::FilePath)]
    pub(crate) test_file: Option<String>,

    /// Clex for generating Tests
    #[arg(short, long, value_hint = ValueHint::Other)]
    pub(crate) generator: Option<String>,

    /// Coding Problem URL
    #[arg(short='u', long, value_hint = ValueHint::Url)]
    pub(crate) problem_url: Option<String>,

    /// Max number of times of iterations
    #[arg(short, long, default_value_t = DEFAULT_ITERATIONS_COUNT, value_hint = ValueHint::Other)]
    pub(crate) iterations: usize,

    /// Continue even after finding one edge case
    #[arg(short, long)]
    pub(crate) no_stop: bool,

    /// Force recompile code even if binaries is up to date
    #[arg(short, long)]
    pub(crate) force_recompile: bool,

    /// Debug mode for verbose output
    #[arg(short, long)]
    pub(crate) debug: bool,
}

#[derive(clap::Args)]
pub(crate) struct GenerateArgs {
    /// Write Clex for generating Tests
    pub(crate) generator: Option<String>,

    /// Copy testcases to clipboard
    #[arg(short, long)]
    pub(crate) clipboard: bool,
}

#[derive(clap::Args)]
pub(crate) struct AiArgs {
    /// Input format
    #[arg(short, long, value_hint = ValueHint::Other)]
    pub(crate) input_format: Option<String>,

    /// Constraints
    #[arg(short, long, value_hint = ValueHint::Other)]
    pub(crate) constraints: Option<String>,

    /// Problem URL
    #[arg(short='u', long, value_hint = ValueHint::Url)]
    pub(crate) problem_url: Option<String>,

    /// Copy clex to clipboard
    #[arg(long)]
    pub(crate) clipboard: bool,
}

impl CpastCommand {
    pub(crate) fn new() -> Option<Self> {
        let opt = Self::parse();

        if let Some(completions) = opt.completions {
            let mut cmd = CpastCommand::command();
            eprintln!("Generating completion file for {completions:?}...");

            print_completions(completions, &mut cmd);

            match completions {
                Shell::Zsh => {
                    eprintln!("\n\n{}\n    {}",
                        "Run the following command below to add it permanently to your shell:".bright_blue(),
                        "cpast --completions=zsh | sudo tee /usr/local/share/zsh/site-functions/_cpast".yellow()
                    );
                }
                Shell::Bash => {
                    eprintln!(
                        "\n\n{}\n    {}",
                        "Run the following command below to add it permanently to your shell:"
                            .bright_blue(),
                        "cpast --completions=bash | sudo tee /etc/bash_completion.d/cpast.bash"
                            .yellow()
                    );
                }
                Shell::Fish => {
                    eprintln!("\n\n{}\n    {}",
                        "Run the following command below to add it permanently to your shell:".bright_blue(),
                        "cpast --completions=fish > ~/.local/share/fish/generated_completions/cpast.fish".yellow()
                    );
                }
                Shell::PowerShell => {
                    eprintln!(
                        "{}\n    {}",
                        "Run the following command below to add it permanently to your shell:"
                            .bright_blue(),
                        "cpast --completions=powershell | Out-File -FilePath $PROFILE -Append"
                            .yellow()
                    );
                }
                // Figure it out yourself XD
                _ => {}
            }

            None
        } else {
            Some(opt)
        }
    }
}
