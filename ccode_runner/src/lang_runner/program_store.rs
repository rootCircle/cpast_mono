use crate::lang_runner::runner::Language;
use crate::utils::file_utils;
use std::path::Path;

use super::runner_error_types::RunnerErrorType;

#[derive(Debug)]
pub struct ProgramStore {
    correct_file: Language,
    test_file: Language,
}

#[derive(Debug)]
enum FileType {
    Correct,
    Test,
}

impl ProgramStore {
    pub fn new(
        correct_file: &Path,
        test_file: &Path,
        do_force_compile: bool,
    ) -> Result<Self, Box<RunnerErrorType>> {
        Ok(ProgramStore {
            correct_file: Language::new(correct_file, do_force_compile)?,
            test_file: Language::new(test_file, do_force_compile)?,
        })
    }

    pub fn run_codes_and_compare_output(
        &self,
        stdin_content: &str,
    ) -> Result<(bool, String, String), Box<RunnerErrorType>> {
        //! Run the code and return the output of the correct and test files  
        //! along with a boolean indicating if the output is different
        //! Output is in the form of (is_different, correct_output, test_output)

        let correct_output =
            self.run_program_code_interface(&self.correct_file, stdin_content, FileType::Correct)?;
        let test_output =
            self.run_program_code_interface(&self.test_file, stdin_content, FileType::Test)?;

        Ok((
            file_utils::string_diff(&correct_output, &test_output),
            correct_output,
            test_output,
        ))
    }

    fn run_program_code_interface(
        &self,
        language: &Language,
        stdin_content: &str,
        file_type: FileType,
    ) -> Result<String, Box<RunnerErrorType>> {
        language
            .run_program_code(stdin_content)
            .map_err(move |err| {
                eprintln!(
                    "[PROGRAM STORE ERROR] Failed to run {:?}!\n{}",
                    file_type, err
                );
                err
            })
    }
}
