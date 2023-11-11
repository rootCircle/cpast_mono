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
    pub fn new(correct_file: &Path, test_file: &Path) -> ProgramStore {

        if !Self::exists(correct_file, test_file) {
            eprintln!("[ERROR] File(s) doesn't exists\nQuitting.....");
            exit(FILE_NOT_FOUND_EXIT_CODE);
        }

        Self {
            correct_file: Language::new(correct_file),
            test_file: Language::new(test_file),
        }
    }

    fn exists(correct_file: &Path, test_file: &Path) -> bool {
        correct_file.exists() && test_file.exists()
    }

    pub fn run_code(&mut self, stdin_content: &str) -> Result<(bool, String, String), &str> {

        let correct_file = self.correct_file.run_program_code(stdin_content);
        let test_file = self.test_file.run_program_code(stdin_content);

        match correct_file {
            Ok(correct_output) => match test_file {
                Ok(test_output) => Ok((
                    file_utils::string_diff(&correct_output, &test_output),
                    correct_output,
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
