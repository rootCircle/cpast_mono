use crate::utils::program_utils;
use crate::utils::program_utils::remake;
use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};
use std::process::exit;

const DEFAULT_PROGRAM_NAME: &str = "program";
const COMPILATION_FAILED_EXIT_CODE: i32 = 2;

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
    pub file_path: PathBuf,
    lang_name: LanguageName,
    compilation_type: CompilationType,
    is_compiled: bool, // For program optimization
    do_force_compile: bool,
}

impl Language {
    pub(crate) fn new(file_path: &Path, do_force_compile: bool) -> Self {
        let lang_name = Self::get_programming_language_name(file_path);

        let compilation_type = Self::get_language_type(&lang_name);

        Self {
            file_path: file_path.to_owned(),
            lang_name,
            compilation_type,
            is_compiled: false,
            do_force_compile,
        }
    }

    pub fn get_programming_language_name(file_path: &Path) -> LanguageName {
        match file_path.extension() {
            Some(file_ext) => match file_ext.to_str() {
                Some("rs") => LanguageName::Rust,
                Some("py") => LanguageName::Python,
                Some("c") => LanguageName::C,
                Some("cpp") => LanguageName::Cpp,
                Some("java") => LanguageName::Java,
                Some("js") => LanguageName::Javascript,
                Some("rb") => LanguageName::Ruby,
                _ => {
                    eprintln!("Unsupported LanguageName {:?}", file_ext);
                    eprintln!("Component: language::LanguageName::detect_program_language");
                    exit(1);
                }
            },
            None => {
                eprintln!("Can't extract the file extension to detect LanguageName");
                eprintln!("Component: language::LanguageName::detect_program_language");
                exit(1);
            }
        }
    }

    fn get_language_type(lang_type: &LanguageName) -> CompilationType {
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

    /// Running single filed self executable program
    pub fn run_program_code(&mut self, stdin_content: &str) -> io::Result<String> {
        match self.compilation_type {
            CompilationType::AheadOfTime => {
                // Need to Compile and then run
                match self.compile_language() {
                    Ok(bin_path) => program_utils::run_program_with_input(
                        &format!("./{}", bin_path),
                        &vec![],
                        stdin_content,
                    ),
                    Err(_e) => exit(COMPILATION_FAILED_EXIT_CODE),
                }
            }
            CompilationType::JustInTime => {
                // Need to Just Run
                match self.run_interpreted_language(stdin_content) {
                    Ok(output) => Ok(output),
                    Err(_err) => {
                        exit(COMPILATION_FAILED_EXIT_CODE);
                    }
                }
            }
            CompilationType::AheadOfTimeInterpreted => {
                // Might require converting to intermediate before running (eg java)
                // Need to Compile and then run
                match self.compile_language() {
                    Ok(bin_path) => match self.file_path.parent() {
                        Some(file_parent) => program_utils::run_program_with_input(
                            "java",
                            &vec!["-cp", file_parent.to_str().unwrap_or(""), &bin_path],
                            stdin_content,
                        ),
                        None => program_utils::run_program_with_input(
                            "java",
                            &vec![&bin_path],
                            stdin_content,
                        ),
                    },
                    Err(_e) => exit(COMPILATION_FAILED_EXIT_CODE),
                }
            }
        }
    }

    fn compile_language(&mut self) -> Result<String, &'static str> {
        // Converts "abc/def.rs" to "def"
        let program_name_stem = match self.file_path.file_stem() {
            Some(program_name) => program_name.to_str().unwrap_or(DEFAULT_PROGRAM_NAME),
            None => DEFAULT_PROGRAM_NAME,
        };

        if self.is_compiled {
            return Ok(program_name_stem.to_string());
        }

        if !self.do_force_compile
            && !remake(self.file_path.clone(), PathBuf::from(program_name_stem))
        {
            self.is_compiled = true;
            return Ok(program_name_stem.to_string());
        }

        let mut program: HashMap<&str, Vec<&str>> = HashMap::new();
        let file_path_str = self.file_path.to_str().unwrap_or("");
        match self.lang_name {
            LanguageName::C => {
                program.insert("gcc", vec!["-o", program_name_stem, file_path_str]);
                program.insert("clang", vec!["-o", program_name_stem, file_path_str]);
            }
            LanguageName::Cpp => {
                program.insert("g++", vec!["-o", program_name_stem, file_path_str]);
                program.insert("clang++", vec!["-o", program_name_stem, file_path_str]);
            }
            LanguageName::Rust => {
                program.insert("rustc", vec!["-o", program_name_stem, file_path_str]);
            }
            LanguageName::Java => {
                program.insert("javac", vec![file_path_str]);
            }
            _ => {
                return Err("Unsupported/Not a Compiled LanguageName");
            }
        }

        for (prog, args) in &program {
            let std_out = program_utils::run_program(prog, args);
            match std_out {
                Ok(_output) => {
                    // println!("{program_name_stem} compiled Successfully with {prog}! \n {}", output);
                    self.is_compiled = true;
                    return Ok(program_name_stem.to_string());
                }
                Err(err) => {
                    eprintln!("WARNING: Failed to compile {program_name_stem} code with {prog} with reason {err}");
                }
            };
        }

        eprintln!("Couldn't Compile the code {program_name_stem}\n");
        Err("Couldn't Compile the code")
    }

    fn run_interpreted_language(&self, stdin_content: &str) -> Result<String, &'static str> {
        let mut program: HashMap<&str, Vec<&str>> = HashMap::new();
        let file_path_str = self.file_path.to_str().unwrap_or("");
        match self.lang_name {
            LanguageName::Python => {
                program.insert("python3", vec![file_path_str]);
                program.insert("python", vec![file_path_str]);
            }
            LanguageName::Ruby => {
                program.insert("ruby", vec![file_path_str]);
            }
            LanguageName::Javascript => {
                program.insert("node", vec![file_path_str]);
                program.insert("deno", vec!["run", file_path_str]);
                program.insert("bun", vec![file_path_str]);
            }
            _ => {
                return Err("Unsupported/Not a Compiled LanguageName");
            }
        };

        for (prog, args) in &program {
            let std_out = program_utils::run_program_with_input(prog, args, stdin_content);
            match std_out {
                Ok(output) => {
                    // println!("{file_path_str} Run Successfully with {prog}!");
                    return Ok(output);
                }
                Err(err) => {
                    eprintln!("[Interpreter] WARNING: Failed to run {file_path_str} code with {prog} with reason {err}");
                }
            };
        }

        Err("Failed to run code!")
    }
}
