use crate::language::Language;
use crate::utils::file_utils;
use std::path::Path;
use std::process::exit;

const FILE_NOT_FOUND_EXIT_CODE: i32 = 6;

#[derive(Debug)]
pub(crate) struct ProgramStore {
    correct_file: Language,
    test_file: Language,
}

impl ProgramStore {
    pub fn new(source_file: &Path, comparing_file: &Path) -> ProgramStore {
        Self {
            correct_file: Language::new(source_file),
            test_file: Language::new(comparing_file),
        }
    }

    fn exists(&self) -> bool {
        self.correct_file.file_path.exists() && self.test_file.file_path.exists()
    }

    pub fn run_code(&mut self, stdin_content: &str) -> Result<(bool, String, String), &str> {
        if !self.exists() {
            eprintln!("[ERROR] File(s) doesn't exists\nQuitting.....");
            exit(FILE_NOT_FOUND_EXIT_CODE);
        }
        let src_file = self.correct_file.run_program_code(stdin_content);
        let test_file = self.test_file.run_program_code(stdin_content);

        match src_file {
            Ok(src_output) => match test_file {
                Ok(test_output) => Ok((
                    file_utils::string_diff(&src_output, &test_output),
                    src_output,
                    test_output,
                )),
                Err(err) => {
                    eprintln!("Failed to run test file!\n{err}");
                    Err("Error running Test File")
                }
            },
            Err(err) => {
                eprintln!("Failed to run source file!\n{err}");
                Err("Error running Source File")
            }
        }
    }
}
