use crate::lang_runner::runner::Language;
use crate::lang_runner::runner_error_types::RunnerErrorType;
use crate::utils::file_utils;
use std::error::Error;
use std::path::Path;

#[derive(Debug)]
pub struct ProgramStore {
    correct_file: Language,
    test_file: Language,
    correct_file_bin_path: String,
    test_file_bin_path: String,
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
    ) -> Result<Self, Box<dyn Error>> {
        if !Self::exists(correct_file, test_file) {
            return Err(Box::new(RunnerErrorType::FileNotFound));
        }

        let mut program_store = ProgramStore {
            correct_file: Language::new(correct_file, do_force_compile)?,
            test_file: Language::new(test_file, do_force_compile)?,
            correct_file_bin_path: String::new(),
            test_file_bin_path: String::new(),
        };

        println!("[PROGRAM STORE INFO] Compiling program/Generating Intermediates");
        program_store.correct_file_bin_path =
            program_store.correct_file.warmup_precompile().unwrap();
        program_store.test_file_bin_path = program_store.test_file.warmup_precompile().unwrap();

        Ok(program_store)
    }

    fn exists(correct_file: &Path, test_file: &Path) -> bool {
        correct_file.exists() && test_file.exists()
    }

    pub fn run_codes_and_compare_output(
        &self,
        stdin_content: &str,
    ) -> Result<(bool, String, String), &str> {
        //! Run the code and return the output of the correct and test files  
        //! along with a boolean indicating if the output is different
        //! Output is in the form of (is_different, correct_output, test_output)

        let correct_output = self.run_program_code_interface(
            &self.correct_file,
            &self.correct_file_bin_path,
            stdin_content,
            FileType::Correct,
        )?;
        let test_output = self.run_program_code_interface(
            &self.test_file,
            &self.test_file_bin_path,
            stdin_content,
            FileType::Test,
        )?;

        Ok((
            file_utils::string_diff(&correct_output, &test_output),
            correct_output,
            test_output,
        ))
    }

    fn run_program_code_interface(
        &self,
        language: &Language,
        bin_path: &str,
        stdin_content: &str,
        file_type: FileType,
    ) -> Result<String, &str> {
        language
            .run_program_code(bin_path, stdin_content)
            .map_err(|err| {
                eprintln!(
                    "[PROGRAM STORE ERROR] Failed to run {:?}!\n{}",
                    file_type, err
                );
                "Error running file"
            })
    }
}
