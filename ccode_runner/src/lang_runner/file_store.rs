use std::{
    error::Error,
    path::{Path, PathBuf},
};

use super::{
    runner::{CompilationType, Language, LanguageName},
    runner_error_types::RunnerErrorType,
};

struct SourceCodeInfo {
    source_path: PathBuf,
    dest_path: Option<PathBuf>,
    language: LanguageName,
}

impl SourceCodeInfo {
    pub fn new(source_file: &Path) -> Result<Self, Box<dyn Error>> {
        if !Self::exists(source_file) {
            return Err(Box::new(RunnerErrorType::FileNotFound));
        }

        let lang = match Language::get_programming_language_name(source_file) {
            Some(lang) => lang,
            None => return Err(Box::new(RunnerErrorType::UnsupportedLanguage)),
        };

        Ok(SourceCodeInfo {
            source_path: source_file.to_path_buf(),
            dest_path: None,
            language: lang,
        })
    }

    pub fn new_from_source_dest(
        source_file: &Path,
        dest_file: &Path,
    ) -> Result<Self, Box<dyn Error>> {
        if !Self::exists(source_file) {
            return Err(Box::new(RunnerErrorType::FileNotFound));
        }

        let lang = match Language::get_programming_language_name(source_file) {
            Some(lang) => lang,
            None => return Err(Box::new(RunnerErrorType::UnsupportedLanguage)),
        };

        let lang_type = Language::get_language_compilation_type(&lang);

        if lang_type == CompilationType::JustInTime {
            return Ok(SourceCodeInfo {
                source_path: source_file.to_path_buf(),
                dest_path: None,
                language: lang,
            });
        }
        Ok(SourceCodeInfo {
            source_path: source_file.to_path_buf(),
            dest_path: Some(dest_file.to_path_buf()),
            language: lang,
        })
    }

    pub fn new_from_text(source_text: &str, lang: LanguageName) -> Result<Self, Box<dyn Error>> {
        todo!("Not implemented yet, create a temp file like mktemp and compile the binary in that dir and then return separate dest as well");
    }

    fn exists(file: &Path) -> bool {
        file.exists()
    }
}
