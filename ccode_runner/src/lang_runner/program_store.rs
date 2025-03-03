//! This module provides functionalities for managing and interacting with stored programs.
//! It includes utilities for running program files.
use crate::lang_runner::runner::Language;
use crate::utils::file_utils;
use std::path::Path;

use super::runner_error_types::RunnerErrorType;

/// A structure that holds two language files for comparison.
///
/// `ProgramStore` maintains references to two files:
/// * A correct implementation file
/// * A test implementation file
///
/// Both files are represented using the `Language` type.
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

    /// Run both correct and test files with the given input and compare their outputs
    ///
    /// # Arguments
    ///
    /// * `stdin_content` - The input content to pass to both programs
    ///
    /// # Returns
    ///
    /// * `Ok((bool, String, String))` - A tuple containing:
    ///   * A boolean indicating if outputs differ (true if different)
    ///   * The output string from the correct file
    ///   * The output string from the test file
    /// * `Err(Box<RunnerErrorType>)` - If there was an error running either program
    pub fn run_codes_and_compare_output(
        &self,
        stdin_content: &str,
    ) -> Result<(bool, String, String), Box<RunnerErrorType>> {
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
