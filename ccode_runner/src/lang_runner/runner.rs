use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::utils::program_utils;
use crate::utils::program_utils::remake;
use core::fmt;
use std::error::Error;
use std::path::{Path, PathBuf};

use super::file_store::SourceCodeInfo;
use super::runner_error_types::RunnerErrorType;

const DEFAULT_PROGRAM_NAME: &str = "program";

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema, PartialEq)]
pub enum LanguageName {
    Python,
    Cpp,
    C,
    Rust,
    Ruby,
    Javascript,
    Java,
}

impl fmt::Display for LanguageName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LanguageName::Python => write!(f, "python"),
            LanguageName::Cpp => write!(f, "cpp"),
            LanguageName::C => write!(f, "c"),
            LanguageName::Rust => write!(f, "rust"),
            LanguageName::Ruby => write!(f, "ruby"),
            LanguageName::Javascript => write!(f, "javascript"),
            LanguageName::Java => write!(f, "java"),
        }
    }
}

impl TryFrom<String> for LanguageName {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "python" => Ok(LanguageName::Python),
            "cpp" => Ok(LanguageName::Cpp),
            "c" => Ok(LanguageName::C),
            "rust" => Ok(LanguageName::Rust),
            "ruby" => Ok(LanguageName::Ruby),
            "javascript" => Ok(LanguageName::Javascript),
            "java" => Ok(LanguageName::Java),
            other => Err(format!(
                "{} is not a supported language. Use either `python`, `cpp`, `c`, `rust`, `ruby`, `javascript` or `java`.",
                other
            )),
        }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum CompilationType {
    Compiled,         // Compiled language like C, C++, Rust, Go, etc.
    Interpreted,      // Interpreted language like Python, etc.
    BytecodeCompiled, // Java, compiled to bytecode, executed by JVM
}

#[derive(Debug)]
pub(crate) struct Language {
    pub(crate) code: SourceCodeInfo,
    is_compiled: bool, // For program optimization
    do_force_compile: bool,
}

impl Language {
    pub(crate) fn new(
        file_path: &Path,
        do_force_compile: bool,
    ) -> Result<Self, Box<RunnerErrorType>> {
        let code = SourceCodeInfo::new(file_path)?;

        let mut lang = Self {
            code,
            is_compiled: false,
            do_force_compile,
        };

        lang.warmup_precompile()?;

        Ok(lang)
    }

    /// One time compilation/intermediate generation before code is actually run for the first time
    fn warmup_precompile(&mut self) -> Result<(), Box<RunnerErrorType>> {
        match self.code.compilation_type {
            CompilationType::Compiled => {
                let compiled_path = self.compile_language()?;
                self.code.register_dest_file(Path::new(&compiled_path))
            }
            // No pre-compilations needed in this case, so return an empty string to signify success
            CompilationType::Interpreted => {}
            // Might require converting to intermediate before running (eg java)
            CompilationType::BytecodeCompiled => {
                let compiled_path = self.compile_language()?;
                self.code.register_dest_file(Path::new(&compiled_path))
            }
        };
        Ok(())
    }

    /// Running single filed self executable program
    pub(crate) fn run_program_code(&self, stdin_content: &str) -> Result<String, Box<dyn Error>> {
        match self.code.compilation_type {
            CompilationType::Compiled => {
                if !self.is_compiled {
                    panic!(
                        "Need to call warmup_precompile() method before run_program_code() is run."
                    );
                }
                Ok(program_utils::run_program_with_input(
                    &format!("./{}", self.code.get_dest_file_str().unwrap()),
                    &vec![],
                    stdin_content,
                )?)
            }
            CompilationType::Interpreted => {
                // Need to Just Run
                Ok(self.run_interpreted_language(stdin_content)?)
            }
            CompilationType::BytecodeCompiled => {
                if !self.is_compiled {
                    panic!(
                        "Need to call warmup_precompile() method before run_program_code() is run."
                    );
                }
                match self.code.language {
                    LanguageName::Java => match self.code.source_path.parent() {
                        Some(file_parent) => Ok(program_utils::run_program_with_input(
                            "java",
                            &vec![
                                "-cp",
                                file_parent.to_str().unwrap_or_default(),
                                self.code.get_dest_file_str().unwrap(),
                            ],
                            stdin_content,
                        )?),
                        None => Ok(program_utils::run_program_with_input(
                            "java",
                            &vec![self.code.get_dest_file_str().unwrap()],
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
            .code
            .source_path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or(DEFAULT_PROGRAM_NAME);

        // Checking if the file is already compiled/doesn't need recompilation
        if self.is_compiled
            || (!self.do_force_compile
                && !remake(&self.code.source_path, &PathBuf::from(program_name_stem))
                    .unwrap_or(true))
        {
            self.is_compiled = true; // Helps a lot in saving time, checking for need for compilations
            return Ok(program_name_stem.to_string());
        }

        let file_path_str = self.code.source_path.to_str().unwrap();
        let compilers = match self.code.language {
            LanguageName::C => vec![
                (
                    "gcc",
                    vec![
                        "-o",
                        program_name_stem,
                        &self.code.source_path.to_str().unwrap(),
                    ],
                ),
                (
                    "clang",
                    vec![
                        "-o",
                        program_name_stem,
                        &self.code.source_path.to_str().unwrap(),
                    ],
                ),
                (
                    "zig",
                    vec![
                        "cc",
                        "-o",
                        program_name_stem,
                        &self.code.source_path.to_str().unwrap(),
                    ],
                ),
            ],
            LanguageName::Cpp => vec![
                (
                    "g++",
                    vec![
                        "-o",
                        program_name_stem,
                        &self.code.source_path.to_str().unwrap(),
                    ],
                ),
                (
                    "clang++",
                    vec![
                        "-o",
                        program_name_stem,
                        &self.code.source_path.to_str().unwrap(),
                    ],
                ),
                (
                    "zig",
                    vec![
                        "c++",
                        "-o",
                        program_name_stem,
                        &self.code.source_path.to_str().unwrap(),
                    ],
                ),
            ],
            LanguageName::Rust => vec![(
                "rustc",
                vec![
                    "-o",
                    program_name_stem,
                    &self.code.source_path.to_str().unwrap(),
                ],
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
        let interpreters = match self.code.language {
            LanguageName::Python => vec![
                ("python3", vec![self.code.source_path.to_str().unwrap()]),
                ("python", vec![self.code.source_path.to_str().unwrap()]),
            ],
            LanguageName::Ruby => vec![("ruby", vec![self.code.source_path.to_str().unwrap()])],
            LanguageName::Javascript => vec![
                ("node", vec![self.code.source_path.to_str().unwrap()]),
                ("deno", vec!["run", self.code.source_path.to_str().unwrap()]),
                ("bun", vec![self.code.source_path.to_str().unwrap()]),
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
                        self.code.source_path.to_str().unwrap(),
                        interpreter,
                        err
                    );
                }
            }
        }

        Err(RunnerErrorType::CodeRunFailed)
    }
}
