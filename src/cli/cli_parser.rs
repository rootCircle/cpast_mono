use clap::Parser;

const DEFAULT_ITERATIONS_COUNT: usize = 5;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    /// The file you have written
    #[arg(short, long, required = true)]
    pub source_file: Option<String>,

    /// File against which you want to do test
    #[arg(short, long, required = true)]
    pub test_file: Option<String>,

    /// Write Generator LanguageName for generating Tests
    #[arg(short, long, required = true)]
    pub(crate) generator: Option<String>,

    /// Number of times to iterate before finding a correct output
    #[arg(short, long, default_value_t = DEFAULT_ITERATIONS_COUNT)]
    pub(crate) iterations: usize,
}

impl CliArgs {
    pub fn new() -> Self {
        CliArgs::parse()
    }
}
