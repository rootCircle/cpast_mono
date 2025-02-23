use crate::utils::program_utils;
use crate::utils::program_utils::remake;
use std::path::{Path, PathBuf};

use super::file_store::SourceCodeInfo;
use super::language_name::{CompilationType, LanguageName};
use super::runner_error_types::RunnerErrorType;

const DEFAULT_PROGRAM_NAME: &str = "program";

#[derive(Debug)]
pub(crate) struct Language {
    pub(crate) code: SourceCodeInfo,
    is_compiled: bool, // For program optimization
    do_force_compile: bool,
}

impl Language {
    pub(crate) async fn new(
        file_path: &Path,
        do_force_compile: bool,
    ) -> Result<Self, Box<RunnerErrorType>> {
        let code = SourceCodeInfo::new(file_path)?;

        let mut lang = Self {
            code,
            is_compiled: false,
            do_force_compile,
        };

        lang.warmup_precompile().await?;

        Ok(lang)
    }

    /// One time compilation/intermediate generation before code is actually run for the first time
    async fn warmup_precompile(&mut self) -> Result<(), Box<RunnerErrorType>> {
        match self.code.compilation_type {
            CompilationType::Compiled => {
                let compiled_path = self.compile_language().await?;
                self.code.register_dest_file(Path::new(&compiled_path))
            }
            // No pre-compilations needed in this case, so return an empty string to signify success
            CompilationType::Interpreted => {}
            // Might require converting to intermediate before running (eg java)
            CompilationType::BytecodeCompiled => {
                let compiled_path = self.compile_language().await?;
                self.code.register_dest_file(Path::new(&compiled_path))
            }
        };
        Ok(())
    }

    /// Running single filed self executable program
    pub(crate) async fn run_program_code(
        &self,
        stdin_content: &str,
    ) -> Result<String, Box<RunnerErrorType>> {
        match self.code.compilation_type {
            CompilationType::Compiled => {
                if !self.is_compiled {
                    return Err(Box::new(RunnerErrorType::WarmupCompileFatal));
                }
                Ok(program_utils::run_program_with_input(
                    &format!("./{}", self.code.get_dest_file_str().unwrap()),
                    &vec![],
                    stdin_content,
                )
                .await
                .map_err(|err| Box::new(RunnerErrorType::ProgramRunError(Box::new(err))))?)
            }
            CompilationType::Interpreted => {
                // Need to Just Run
                Ok(self.run_interpreted_language(stdin_content).await?)
            }
            CompilationType::BytecodeCompiled => {
                if !self.is_compiled {
                    return Err(Box::new(RunnerErrorType::WarmupCompileFatal));
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
                        )
                        .await
                        .map_err(|err| {
                            Box::new(RunnerErrorType::ProgramRunError(Box::new(err)))
                        })?),
                        None => Ok(program_utils::run_program_with_input(
                            "java",
                            &vec![self.code.get_dest_file_str().unwrap()],
                            stdin_content,
                        )
                        .await
                        .map_err(|err| {
                            Box::new(RunnerErrorType::ProgramRunError(Box::new(err)))
                        })?),
                    },
                    _ => Err(Box::new(RunnerErrorType::InvalidLanguageMapping(
                        self.code.language.clone(),
                        self.code.compilation_type.clone(),
                    ))),
                }
            }
        }
    }

    async fn compile_language(&mut self) -> Result<String, RunnerErrorType> {
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
            _ => {
                return Err(RunnerErrorType::InvalidCompilationMapping(
                    self.code.language.clone(),
                ));
            }
        };

        for (compiler, args) in compilers {
            let std_out = program_utils::run_program(compiler, &args).await;
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

        Err(RunnerErrorType::CodeRunFailed(
            self.code.source_path.to_path_buf(),
        ))
    }

    async fn run_interpreted_language(
        &self,
        stdin_content: &str,
    ) -> Result<String, RunnerErrorType> {
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
            _ => {
                return Err(RunnerErrorType::InvalidLanguageMapping(
                    self.code.language.clone(),
                    self.code.compilation_type.clone(),
                ));
            }
        };

        for (interpreter, args) in interpreters {
            let std_out =
                program_utils::run_program_with_input(interpreter, &args, stdin_content).await;
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

        Err(RunnerErrorType::CodeRunFailed(
            self.code.source_path.to_path_buf(),
        ))
    }
}
