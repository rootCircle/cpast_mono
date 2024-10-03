use crate::utils::program_utils;
use crate::utils::program_utils::remake;
use std::error::Error;
use std::path::{Path, PathBuf};

use super::runner_error_types::RunnerErrorType;

const DEFAULT_PROGRAM_NAME: &str = "program";
const EMPTY_STRING: &str = "";

#[derive(Debug)]
pub(crate) enum LanguageName {
    Python,
    Cpp,
    C,
    Rust,
    Ruby,
    Javascript,
    Java,
}

#[derive(Debug)]
enum CompilationType {
    AheadOfTime,            // Compiled language like C, C++, Rust, Java, Go etc
    JustInTime,             // Python etc
    AheadOfTimeInterpreted, // Java
}

#[derive(Debug)]
pub(crate) struct Language {
    pub(crate) file_path: PathBuf,
    lang_name: LanguageName,
    compilation_type: CompilationType,
    is_compiled: bool, // For program optimization
    do_force_compile: bool,
}

impl Language {
    pub(crate) fn new(file_path: &Path, do_force_compile: bool) -> Result<Self, RunnerErrorType> {
        let lang_name = match Self::get_programming_language_name(file_path) {
            Some(lang) => lang,
            None => {
                return Err(RunnerErrorType::UnsupportedLanguage);
            }
        };
        let compilation_type = Self::get_language_compilation_type(&lang_name);

        Ok(Self {
            file_path: file_path.to_owned(),
            lang_name,
            compilation_type,
            is_compiled: false,
            do_force_compile,
        })
    }

    fn get_programming_language_name(file_path: &Path) -> Option<LanguageName> {
        match file_path.extension().and_then(|ext| ext.to_str()) {
            Some("rs") => Some(LanguageName::Rust),
            Some("py") => Some(LanguageName::Python),
            Some("c") => Some(LanguageName::C),
            Some("cpp") | Some("cxx") | Some("c++") | Some("cc") | Some("C") => {
                Some(LanguageName::Cpp)
            }
            Some("java") => Some(LanguageName::Java),
            Some("js") => Some(LanguageName::Javascript),
            Some("rb") => Some(LanguageName::Ruby),
            _ => None,
        }
    }

    fn get_language_compilation_type(lang_type: &LanguageName) -> CompilationType {
        match lang_type {
            LanguageName::Rust | LanguageName::Cpp | LanguageName::C => {
                CompilationType::AheadOfTime
            }
            LanguageName::Python | LanguageName::Ruby | LanguageName::Javascript => {
                CompilationType::JustInTime
            }
            LanguageName::Java => CompilationType::AheadOfTimeInterpreted,
        }
    }

    /// One time compilation/intermediate generation before code is actually run for the first time
    pub(crate) fn warmup_precompile(&mut self) -> Result<String, Box<dyn Error>> {
        Ok(match self.compilation_type {
            CompilationType::AheadOfTime => self.compile_language()?,
            // No pre-compilations needed in this case, so return an empty string to signify success
            CompilationType::JustInTime => EMPTY_STRING.to_owned(),
            // Might require converting to intermediate before running (eg java)
            CompilationType::AheadOfTimeInterpreted => self.compile_language()?,
        })
    }

    /// Running single filed self executable program
    pub(crate) fn run_program_code(
        &self,
        bin_path: &str,
        stdin_content: &str,
    ) -> Result<String, Box<dyn Error>> {
        match self.compilation_type {
            CompilationType::AheadOfTime => {
                if !self.is_compiled {
                    panic!(
                        "Need to call warmup_precompile() method before run_program_code() is run."
                    );
                }
                Ok(program_utils::run_program_with_input(
                    &format!("./{}", bin_path),
                    &vec![],
                    stdin_content,
                )?)
            }
            CompilationType::JustInTime => {
                // Need to Just Run
                Ok(self.run_interpreted_language(stdin_content)?)
            }
            CompilationType::AheadOfTimeInterpreted => {
                if !self.is_compiled {
                    panic!(
                        "Need to call warmup_precompile() method before run_program_code() is run."
                    );
                }
                match self.lang_name {
                    LanguageName::Java => match self.file_path.parent() {
                        Some(file_parent) => Ok(program_utils::run_program_with_input(
                            "java",
                            &vec!["-cp", file_parent.to_str().unwrap_or_default(), bin_path],
                            stdin_content,
                        )?),
                        None => Ok(program_utils::run_program_with_input(
                            "java",
                            &vec![bin_path],
                            stdin_content,
                        )?),
                    },
                    _ => Err(Box::new(RunnerErrorType::UnsupportedLanguage)),
                }
            }
        }
    }

    fn compile_language(&mut self) -> Result<String, RunnerErrorType> {
        let program_name_stem = self
            .file_path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or(DEFAULT_PROGRAM_NAME);

        // Checking if the file is already compiled/doesn't need recompilation
        if self.is_compiled
            || (!self.do_force_compile
                && !remake(&self.file_path, &PathBuf::from(program_name_stem)).unwrap_or(true))
        {
            self.is_compiled = true; // Helps a lot in saving time, checking for need for compilations
            return Ok(program_name_stem.to_string());
        }

        let file_path_str = self.file_path.to_str().unwrap_or("");
        let compilers = match self.lang_name {
            LanguageName::C => vec![
                (
                    "gcc",
                    vec!["-o", program_name_stem, &self.file_path.to_str().unwrap()],
                ),
                (
                    "clang",
                    vec!["-o", program_name_stem, &self.file_path.to_str().unwrap()],
                ),
                (
                    "zig",
                    vec![
                        "cc",
                        "-o",
                        program_name_stem,
                        &self.file_path.to_str().unwrap(),
                    ],
                ),
            ],
            LanguageName::Cpp => vec![
                (
                    "g++",
                    vec!["-o", program_name_stem, &self.file_path.to_str().unwrap()],
                ),
                (
                    "clang++",
                    vec!["-o", program_name_stem, &self.file_path.to_str().unwrap()],
                ),
                (
                    "zig",
                    vec![
                        "c++",
                        "-o",
                        program_name_stem,
                        &self.file_path.to_str().unwrap(),
                    ],
                ),
            ],
            LanguageName::Rust => vec![(
                "rustc",
                vec!["-o", program_name_stem, &self.file_path.to_str().unwrap()],
            )],
            LanguageName::Java => vec![("javac", vec![file_path_str])],
            _ => return Err(RunnerErrorType::UnsupportedLanguage),
        };

        for (compiler, args) in compilers {
            let std_out = program_utils::run_program(compiler, &args);
            match std_out {
                Ok(_) => {
                    self.is_compiled = true;
                    return Ok(program_name_stem.to_string());
                }
                Err(err) => {
                    eprintln!(
                        "[RUNNER WARNING] Failed to compile {} code with {} with reason {}",
                        program_name_stem, compiler, err
                    );
                }
            }
        }

        eprintln!(
            "[RUNNER ERROR] Couldn't compile the code {}.",
            program_name_stem
        );
        Err(RunnerErrorType::CodeRunFailed)
    }

    fn run_interpreted_language(&self, stdin_content: &str) -> Result<String, RunnerErrorType> {
        let interpreters = match self.lang_name {
            LanguageName::Python => vec![
                ("python3", vec![self.file_path.to_str().unwrap()]),
                ("python", vec![self.file_path.to_str().unwrap()]),
            ],
            LanguageName::Ruby => vec![("ruby", vec![self.file_path.to_str().unwrap()])],
            LanguageName::Javascript => vec![
                ("node", vec![self.file_path.to_str().unwrap()]),
                ("deno", vec!["run", self.file_path.to_str().unwrap()]),
                ("bun", vec![self.file_path.to_str().unwrap()]),
            ],
            _ => return Err(RunnerErrorType::UnsupportedLanguage),
        };

        for (interpreter, args) in interpreters {
            let std_out = program_utils::run_program_with_input(interpreter, &args, stdin_content);
            match std_out {
                Ok(output) => {
                    return Ok(output);
                }
                Err(err) => {
                    eprintln!(
                        "[INTERPRETER WARNING] Failed to run {} code with {} with reason {}",
                        self.file_path.to_str().unwrap(),
                        interpreter,
                        err
                    );
                }
            }
        }

        Err(RunnerErrorType::CodeRunFailed)
    }
}
