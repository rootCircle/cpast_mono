use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use tempfile::Builder;

use crate::utils::java_classname::get_java_public_classname_from_text;

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
        if compilation_type != CompilationType::Interpreted {
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
                temp_dir: Some(temp_dir.keep()),
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
        self.dest_path.as_ref().and_then(|dest| dest.to_str())
    }

    /// Creates a new `SourceCodeInfo` instance with an optional custom destination path.
    ///
    /// This method initializes source code information for both interpreted and compiled languages,
    /// with special handling for Java source files. If the dest_file is None, then it automatically creates a temp file for output.
    ///
    /// # Arguments
    ///
    /// * `source_file` - Path to the source code file
    /// * `dest_file` - Optional custom destination path for the compiled output
    ///
    /// # Returns
    ///
    /// * `Result<SourceCodeInfo, Box<RunnerErrorType>>` - A Result containing either:
    ///   * `SourceCodeInfo` - Successfully created source code information
    ///   * `Box<RunnerErrorType>` - Error encountered during creation
    ///
    /// # Errors
    ///
    /// Returns an error in the following cases:
    /// * Source file does not exist
    /// * Language is not supported
    /// * For Java files, when destination filename doesn't match source filename
    /// * File stem extraction fails
    /// * Temporary directory creation fails
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

        let compilation_type = get_language_compilation_type(&lang);

        if compilation_type == CompilationType::Interpreted {
            return Ok(SourceCodeInfo {
                source_path: source_file.to_path_buf(),
                dest_path: None,
                compilation_type,
                language: lang,
                temp_dir: None,
            });
        }

        let dest_file = match dest_file {
            Some(dest) => {
                if lang == LanguageName::Java {
                    let source_filename_stem = source_file
                        .file_stem()
                        .and_then(|stem| stem.to_str())
                        .unwrap_or(DEFAULT_PROGRAM_NAME);

                    let dest_filename_stem = dest_file
                        .and_then(|dest| dest.file_stem())
                        .and_then(|stem| stem.to_str())
                        .ok_or(Box::new(RunnerErrorType::FileStemExtractionError(
                            source_file.to_path_buf(),
                        )))?;

                    if dest_filename_stem != source_filename_stem {
                        return Err(Box::new(RunnerErrorType::JavaMismatchDestinationFile(
                            dest_file.map(|dest| dest.to_path_buf()),
                        )));
                    }
                }
                dest.to_path_buf()
            }
            None => {
                let program_name_stem = source_file
                    .file_stem()
                    .and_then(|stem| stem.to_str())
                    .unwrap_or(DEFAULT_PROGRAM_NAME);

                let temp_dir = Builder::new()
                    .prefix("cpast_runner_")
                    .tempdir()
                    .map_err(|e| Box::new(RunnerErrorType::FileCreationError(Box::new(e))))?;

                temp_dir.path().join(program_name_stem)
            }
        };

        Ok(SourceCodeInfo {
            source_path: source_file.to_path_buf(),
            dest_path: Some(dest_file),
            compilation_type,
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
        let source_file_stem = if lang == LanguageName::Java {
            // Java has some strict requirements around filenaming
            get_java_public_classname_from_text(source_text).ok_or(Box::new(
                RunnerErrorType::JavaNoPublicClassFound(source_text.to_owned()),
            ))?
        } else {
            format!("source_{}", rand::random::<u16>())
        };

        let source_file_path = temp_dir
            .path()
            .join(format!("{source_file_stem}.{file_extension}"));

        let mut source_file_handle = File::create(&source_file_path)
            .map_err(|e| Box::new(RunnerErrorType::FileCreationError(Box::new(e))))?;

        source_file_handle
            .write_all(source_text.as_bytes())
            .map_err(|e| Box::new(RunnerErrorType::FileCreationError(Box::new(e))))?;
        source_file_handle
            .flush()
            .map_err(|e| Box::new(RunnerErrorType::FileCreationError(Box::new(e))))?;

        let source_path = source_file_path.to_path_buf();
        let dest_path = if compilation_type == CompilationType::Compiled {
            Some(temp_dir.path().join("executable"))
        } else if compilation_type == CompilationType::BytecodeCompiled {
            let program_name_stem = source_file_path
                .file_stem()
                .and_then(|stem| stem.to_str())
                .unwrap_or(DEFAULT_PROGRAM_NAME);
            let dest_path = temp_dir.path().join(program_name_stem);
            Some(dest_path)
        } else {
            None
        };

        Ok(SourceCodeInfo {
            source_path,
            dest_path,
            compilation_type,
            language: lang,
            temp_dir: Some(temp_dir.keep()),
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
