use std::path::{Path, PathBuf};

use super::{
    language_name::{
        CompilationType, LanguageName, get_language_compilation_type, get_programming_language_name,
    },
    runner_error_types::RunnerErrorType,
};

#[derive(Debug)]
pub(crate) struct SourceCodeInfo {
    pub(crate) source_path: PathBuf,
    /// If the language is interpreted, then the dest_path will be None
    pub(crate) dest_path: Option<PathBuf>,
    pub(crate) language: LanguageName,
    pub(crate) compilation_type: CompilationType,
}

impl SourceCodeInfo {
    pub fn new(source_file: &Path) -> Result<Self, Box<RunnerErrorType>> {
        if !Self::exists(source_file) {
            return Err(Box::new(RunnerErrorType::FileNotFound(
                source_file.to_path_buf(),
            )));
        }

        let lang = match get_programming_language_name(source_file) {
            Some(lang) => lang,
            None => {
                return Err(Box::new(RunnerErrorType::UnsupportedLanguage(
                    source_file.to_path_buf(),
                )));
            }
        };

        Ok(SourceCodeInfo {
            source_path: source_file.to_path_buf(),
            dest_path: None,
            compilation_type: get_language_compilation_type(&lang),
            language: lang,
        })
    }

    pub fn register_dest_file(&mut self, dest_file: &Path) {
        self.dest_path = Some(dest_file.to_path_buf());
    }

    pub fn get_dest_file_str(&self) -> Option<&str> {
        match &self.dest_path {
            Some(dest) => Some(dest.to_str().unwrap()),
            None => None,
        }
    }

    #[allow(dead_code)]
    pub fn new_from_source_dest(
        source_file: &Path,
        dest_file: Option<&Path>,
    ) -> Result<Self, Box<RunnerErrorType>> {
        if !Self::exists(source_file) {
            return Err(Box::new(RunnerErrorType::FileNotFound(
                source_file.to_path_buf(),
            )));
        }

        let lang = match get_programming_language_name(source_file) {
            Some(lang) => lang,
            None => {
                return Err(Box::new(RunnerErrorType::UnsupportedLanguage(
                    source_file.to_path_buf(),
                )));
            }
        };

        let lang_type = get_language_compilation_type(&lang);

        if lang_type == CompilationType::Interpreted {
            return Ok(SourceCodeInfo {
                source_path: source_file.to_path_buf(),
                dest_path: None,
                compilation_type: lang_type,
                language: lang,
            });
        }
        Ok(SourceCodeInfo {
            source_path: source_file.to_path_buf(),
            dest_path: dest_file.map(|dest| dest.to_path_buf()),
            compilation_type: lang_type,
            language: lang,
        })
    }

    #[allow(dead_code)]
    pub fn new_from_text(
        _source_text: &str,
        _lang: LanguageName,
    ) -> Result<Self, Box<RunnerErrorType>> {
        unimplemented!(
            "Not implemented yet, create a temp file like mktemp and compile the binary in that dir and then return separate dest as well"
        );
    }

    fn exists(file: &Path) -> bool {
        file.exists()
    }
}
