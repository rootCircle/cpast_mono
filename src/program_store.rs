use std::path::Path;
use std::process::exit;
use clap::builder::Str;
use crate::language::Language;
use crate::utils::file_utils;

const FILE_NOT_FOUND_EXIT_CODE: i32 = 6;

#[derive(Debug)]
pub(crate) struct ProgramStore<'a> {
    source_file: &'a Path,
    test_file: &'a Path,
    source_lang: Option<Language>,
    test_lang: Option<Language>
}

impl<'a> ProgramStore<'a> {
    pub fn new(source_file: &'a Path, comparing_file: &'a Path) -> ProgramStore<'a> {
        Self {
            source_file,
            test_file: comparing_file,
            source_lang: Language::get_programming_language_name(source_file),
            test_lang: Language::get_programming_language_name(comparing_file),
        }
    }

    fn exists(&self) -> bool {
        self.source_file.exists() && self.test_file.exists()
    }

    pub fn run_code(&self, stdin_content: &str) -> Result<(bool, String, String), &str> {
        if !self.exists() {
            eprintln!("[ERROR] File(s) doesn't exists\nQuitting.....");
            exit(FILE_NOT_FOUND_EXIT_CODE);
        }
        let src_file = Language::run_program_code(self.source_file, &self.source_lang, stdin_content);
        let test_file = Language::run_program_code(self.test_file, &self.test_lang, stdin_content);

        match src_file {
            Ok(src_output) => {
                match test_file  {
                    Ok(test_output) => {
                        Ok((file_utils::string_diff(&src_output, &test_output), src_output, test_output))
                    }
                    Err(err) => {
                        eprintln!("Failed to run test file!\n{err}");
                        Err("Error running Test File")
                    }
                }
            },
            Err(err) => {
                eprintln!("Failed to run source file!\n{err}");
                Err("Error running Source File")
            }
        }
    }
}
