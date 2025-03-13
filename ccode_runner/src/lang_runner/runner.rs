use crate::utils::program_utils;
use crate::utils::program_utils::remake;
use std::path::{Path, PathBuf};

use super::file_store::SourceCodeInfo;
use super::language_name::{CompilationType, LanguageName};
use super::runner_error_types::RunnerErrorType;

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

        // One time compilation/intermediate generation before code is actually run for the first time
        // For intreperted languages, no need to compile
        // For bytecode compiled languages, compile to bytecode as it might require intermediate compilation (eg Java)
        lang.compile_language()?;

        Ok(lang)
    }

    /// Running single filed self executable program
    pub(crate) fn run_program_code(
        &self,
        stdin_content: &str,
    ) -> Result<String, Box<RunnerErrorType>> {
        match self.code.compilation_type {
            CompilationType::Compiled => {
                if !self.is_compiled {
                    return Err(Box::new(RunnerErrorType::WarmupCompileFatal));
                }
                Ok(program_utils::run_program_with_input(
                    self.code.get_dest_file_str().unwrap(),
                    &vec![],
                    stdin_content,
                )
                .map_err(|err| Box::new(RunnerErrorType::ProgramRunError(Box::new(err))))?)
            }
            CompilationType::Interpreted => {
                // Need to Just Run
                Ok(self.run_interpreted_language(stdin_content)?)
            }
            CompilationType::BytecodeCompiled => {
                if !self.is_compiled {
                    return Err(Box::new(RunnerErrorType::WarmupCompileFatal));
                }

                match self.code.language {
                    LanguageName::Java => Ok(program_utils::run_program_with_input(
                        "java",
                        &vec![
                            "-cp",
                            self.code
                                .temp_dir
                                .as_ref()
                                .unwrap()
                                .to_str()
                                .unwrap_or_default(),
                            self.code.source_path.file_stem().unwrap().to_str().unwrap(),
                        ],
                        stdin_content,
                    )
                    .map_err(|err| Box::new(RunnerErrorType::ProgramRunError(Box::new(err))))?),
                    _ => Err(Box::new(RunnerErrorType::InvalidLanguageMapping(
                        self.code.language.clone(),
                        self.code.compilation_type.clone(),
                    ))),
                }
            }
        }
    }

    fn compile_language(&mut self) -> Result<(), RunnerErrorType> {
        if self.code.compilation_type != CompilationType::Compiled
            && self.code.compilation_type != CompilationType::BytecodeCompiled
        {
            return Ok(()); // No compilation needed
        }

        let dest_file = match &self.code.dest_path {
            Some(dest_path) => dest_path,
            None => {
                return Err(RunnerErrorType::EmptyDestinationPath(
                    self.code.source_path.to_path_buf(),
                    self.code.language.clone(),
                    self.code.compilation_type.clone(),
                ));
            }
        };

        // Checking if the file is already compiled/doesn't need recompilation
        if self.is_compiled
            || (!self.do_force_compile
                && !remake(&self.code.source_path, &PathBuf::from(dest_file)).unwrap_or(true))
        {
            self.is_compiled = true; // Helps a lot in saving time, checking for need for compilations
            return Ok(());
        }

        let file_path_str = self.code.source_path.to_str().unwrap();
        let compilers = match self.code.language {
            LanguageName::C => vec![
                (
                    "gcc",
                    vec![
                        "-o",
                        dest_file.to_str().unwrap(),
                        &self.code.source_path.to_str().unwrap(),
                    ],
                ),
                (
                    "clang",
                    vec![
                        "-o",
                        dest_file.to_str().unwrap(),
                        &self.code.source_path.to_str().unwrap(),
                    ],
                ),
                (
                    "zig",
                    vec![
                        "cc",
                        "-o",
                        dest_file.to_str().unwrap(),
                        &self.code.source_path.to_str().unwrap(),
                    ],
                ),
            ],
            LanguageName::Cpp => vec![
                (
                    "g++",
                    vec![
                        "-o",
                        dest_file.to_str().unwrap(),
                        &self.code.source_path.to_str().unwrap(),
                    ],
                ),
                (
                    "clang++",
                    vec![
                        "-o",
                        dest_file.to_str().unwrap(),
                        &self.code.source_path.to_str().unwrap(),
                    ],
                ),
                (
                    "zig",
                    vec![
                        "c++",
                        "-o",
                        dest_file.to_str().unwrap(),
                        &self.code.source_path.to_str().unwrap(),
                    ],
                ),
            ],
            LanguageName::Rust => vec![(
                "rustc",
                vec![
                    "-o",
                    dest_file.to_str().unwrap(),
                    &self.code.source_path.to_str().unwrap(),
                ],
            )],
            LanguageName::Java => vec![(
                "javac",
                vec![
                    "-d",
                    self.code.temp_dir.as_ref().unwrap().to_str().unwrap(),
                    file_path_str,
                ],
            )],
            _ => {
                return Err(RunnerErrorType::InvalidCompilationMapping(
                    self.code.language.clone(),
                ));
            }
        };

        for (compiler, args) in compilers {
            let std_out = program_utils::run_program(compiler, &args);
            match std_out {
                Ok(_) => {
                    self.is_compiled = true;
                    return Ok(());
                }
                Err(err) => {
                    eprintln!(
                        "[RUNNER WARNING] Failed to compile {:?} code with {} with reason {}",
                        dest_file, compiler, err
                    );
                }
            }
        }

        Err(RunnerErrorType::CodeRunFailed(
            self.code.source_path.to_path_buf(),
        ))
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
            _ => {
                return Err(RunnerErrorType::InvalidLanguageMapping(
                    self.code.language.clone(),
                    self.code.compilation_type.clone(),
                ));
            }
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

        Err(RunnerErrorType::CodeRunFailed(
            self.code.source_path.to_path_buf(),
        ))
    }
}
