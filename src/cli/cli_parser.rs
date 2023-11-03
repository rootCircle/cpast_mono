use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct CliArgs {
    #[arg(short, long, required = true)]
    pub(crate) source_file: Option<String>,

    #[arg(short, long, required = true)]
    pub(crate) test_file: Option<String>,
}

impl CliArgs {
    pub fn new() -> Self {
        CliArgs::parse()
    }
}
