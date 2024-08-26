use std::process::exit;

pub(crate) enum RunnerErrorType {
    UnsupportedLanguage,
    CodeRunFailed,
}

impl RunnerErrorType {
    pub fn print_and_exit(&self, exit_code: i32) -> ! {
        eprintln!("{}", self.get_msg());
        exit(exit_code);
    }

    fn get_msg(&self) -> String {
        match self {
            RunnerErrorType::UnsupportedLanguage => String::from("Unsupported language"),
            RunnerErrorType::CodeRunFailed => String::from("Code run failed"),
        }
    }
}
