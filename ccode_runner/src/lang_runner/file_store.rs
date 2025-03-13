use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use tempfile::Builder;

use super::language_name::get_file_extension;
use super::{
    language_name::{
        CompilationType, LanguageName, get_language_compilation_type, get_programming_language_name,
    },
    runner_error_types::RunnerErrorType,
};

const DEFAULT_PROGRAM_NAME: &str = "program";

#[derive(Debug)]
pub(crate) struct SourceCodeInfo {
    pub(crate) source_path: PathBuf,
    /// If the language is interpreted, then the dest_path will be None
    pub(crate) dest_path: Option<PathBuf>,
    pub(crate) language: LanguageName,
    pub(crate) compilation_type: CompilationType,
    pub(crate) temp_dir: Option<PathBuf>,
}

impl SourceCodeInfo {
    pub(crate) fn new(source_file: &Path) -> Result<Self, Box<RunnerErrorType>> {
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

        let compilation_type = get_language_compilation_type(&lang);
        if compilation_type == CompilationType::Compiled
            || compilation_type == CompilationType::BytecodeCompiled
        {
            let program_name_stem = source_file
                .file_stem()
                .and_then(|stem| stem.to_str())
                .unwrap_or(DEFAULT_PROGRAM_NAME);

            let temp_dir = Builder::new()
                .prefix("cpast_runner_")
                .tempdir()
                .map_err(|e| Box::new(RunnerErrorType::FileCreationError(Box::new(e))))?;

            let dest_path = temp_dir.path().join(program_name_stem);

            return Ok(SourceCodeInfo {
                source_path: source_file.to_path_buf(),
                dest_path: Some(dest_path),
                compilation_type,
                language: lang,
                temp_dir: Some(temp_dir.into_path()),
            });
        }

        Ok(SourceCodeInfo {
            source_path: source_file.to_path_buf(),
            dest_path: None,
            compilation_type,
            language: lang,
            temp_dir: None,
        })
    }

    pub(crate) fn get_dest_file_str(&self) -> Option<&str> {
        match &self.dest_path {
            Some(dest) => Some(dest.to_str().unwrap()),
            None => None,
        }
    }

    pub(crate) fn new_from_custom_dest(
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
                temp_dir: None,
            });
        }
        Ok(SourceCodeInfo {
            source_path: source_file.to_path_buf(),
            dest_path: dest_file.map(|dest| dest.to_path_buf()),
            compilation_type: lang_type,
            language: lang,
            temp_dir: None,
        })
    }

    pub(crate) fn new_from_text(
        source_text: &str,
        lang: LanguageName,
    ) -> Result<Self, Box<RunnerErrorType>> {
        let compilation_type = get_language_compilation_type(&lang);

        let temp_dir = Builder::new()
            .prefix("cpast_runner_")
            .tempdir()
            .map_err(|e| Box::new(RunnerErrorType::FileCreationError(Box::new(e))))?;

        let file_extension = get_file_extension(&lang);

        let source_file = Builder::new()
            .prefix("source_")
            .suffix(&format!(".{}", file_extension))
            .tempfile_in(&temp_dir)
            .map_err(|e| Box::new(RunnerErrorType::FileCreationError(Box::new(e))))?;
        let source_path = source_file.path().to_path_buf();

        let mut file = File::create(&source_path)
            .map_err(|e| Box::new(RunnerErrorType::FileCreationError(Box::new(e))))?;
        file.write_all(source_text.as_bytes())
            .map_err(|e| Box::new(RunnerErrorType::FileCreationError(Box::new(e))))?;

        let dest_path = if compilation_type == CompilationType::Compiled {
            Some(temp_dir.path().join("executable"))
        } else {
            None
        };

        Ok(SourceCodeInfo {
            source_path,
            dest_path,
            compilation_type,
            language: lang,
            temp_dir: Some(temp_dir.into_path()),
        })
    }

    fn exists(file: &Path) -> bool {
        file.exists()
    }
}

impl Drop for SourceCodeInfo {
    fn drop(&mut self) {
        if let Some(temp_dir) = &self.temp_dir {
            // DANGEROUS: This will delete the temp directory and all its contents
            // Use with caution
            let _ = std::fs::remove_dir_all(temp_dir);
        }
    }
}
