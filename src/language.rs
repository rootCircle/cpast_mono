use std::collections::HashMap;
use std::path::Path;
use std::process::exit;
use std::io;
use crate::utils::program_utils;

const DEFAULT_PROGRAM_NAME: &str = "program";
const UNSUPPORTED_LANGUAGE_EXIT_CODE: i32 = 2;
const COMPILATION_FAILED_EXIT_CODE: i32 = 2;

#[derive(Debug)]
pub(crate) enum Language {
    Python,
    Cpp,
    C,
    Rust,
    Ruby,
    Javascript,
    Java
}

enum CompilationType {
    AheadOfTime, // Compiled language like C, C++, Rust, Java, Go etc
    JustInTime,  // Python etc
    AheadOfTimeIntrepreted, // Java
}

impl Language {
    pub fn get_programming_language_name(file_path: &Path) -> Option<Language> {
        match file_path.extension() {
            Some(src_str) => match src_str.to_str() {
                Some("rs") => Some(Language::Rust),
                Some("py") => Some(Language::Python),
                Some("c") => Some(Language::C),
                Some("cpp") => Some(Language::Cpp),
                Some("java") => Some(Language::Java),
                Some("js") => Some(Language::Javascript),
                Some("rb") => Some(Language::Ruby),
                _ => None,
            },
            None => None,
        }
    }

    fn get_language_type(lang_type: &Language) -> CompilationType {
        match lang_type {
            Language::Rust | Language::Cpp | Language::C => CompilationType::AheadOfTime,
            Language::Python | Language::Ruby | Language::Javascript => CompilationType::JustInTime,
            Language::Java => CompilationType::AheadOfTimeIntrepreted
        }
    }

    /// Running single filed self executable program
    pub fn run_program_code(file_path: &Path, lang_name: &Option<Language>, stdin_content: &str) -> io::Result<String> {
        match lang_name {
            Some(lang) => {
                let lang_type = Self::get_language_type(lang);
                match lang_type {
                    CompilationType::AheadOfTime => {
                        // Need to Compile and then run
                        match  Self::compile_language(file_path, lang) {
                            Ok(bin_path) => {
                                program_utils::run_program_with_input(&format!("./{}", bin_path), &vec![], stdin_content)
                            },
                            Err(_e) => {
                                exit(COMPILATION_FAILED_EXIT_CODE)
                            }
                        }
                    }
                    CompilationType::JustInTime => {
                        // Need to Just Run
                        match  Self::run_intrepreted_language(file_path, lang, stdin_content) {
                            Ok(output) => {
                                Ok(output)
                            },
                            Err(_err) => {
                                exit(COMPILATION_FAILED_EXIT_CODE);
                            }
                        }
                    }
                    CompilationType::AheadOfTimeIntrepreted => {
                        // Might require converting to intermediate before running (eg java)
                        // Need to Compile and then run
                        match  Self::compile_language(file_path, lang) {
                            Ok(bin_path) => {
                                match file_path.parent() {
                                    Some(file_parent) => {
                                        program_utils::run_program_with_input("java", &vec!["-cp", file_parent.to_str().unwrap_or(""), &bin_path], stdin_content)
                                    },
                                    None => {
                                        program_utils::run_program_with_input("java", &vec![&bin_path], stdin_content)
                                    }
                                }

                            },
                            Err(_e) => {
                                exit(COMPILATION_FAILED_EXIT_CODE)
                            }
                        }
                    }
                }
            }
            None => {
                eprintln!("Unsupported Language {:?}", lang_name);
                eprintln!("Component: language::Language::detect_program_language");
                std::process::exit(1);
            }
        }
    }

    fn compile_language(file_path: &Path, lang_name: &Language) -> Result<String, &'static str> {

        // Converts "abc/def.rs" to "def"
        let prog_name_stem = match file_path.file_stem() {
            Some(prog_name) => match prog_name.to_str() {
                Some(t) => t,
                None => DEFAULT_PROGRAM_NAME
            },
            None => DEFAULT_PROGRAM_NAME
        };

        let mut program: HashMap<&str, Vec<&str>> = HashMap::new();

        match lang_name {
            Language::C => {
                program.insert("gcc", vec!["-o", prog_name_stem, file_path.to_str().unwrap_or("")]);
                program.insert("clang", vec!["-o", prog_name_stem, file_path.to_str().unwrap_or("")]);
            },
            Language::Cpp => {
                program.insert("g++", vec!["-o", prog_name_stem, file_path.to_str().unwrap_or("")]);
                program.insert("clang++", vec!["-o", prog_name_stem, file_path.to_str().unwrap_or("")]);
            },
            Language::Rust => {
                program.insert("rustc", vec!["-o", prog_name_stem, file_path.to_str().unwrap_or("")]);
            },
            Language::Java => {
                program.insert("javac", vec![file_path.to_str().unwrap_or("")]);
            }
            _ => {
                return Err("Unsupported/Not a Compiled Language");
            }
        }

        for (prog, args) in &program {
            let std_out = program_utils::run_program(prog, args);
            match std_out {
                Ok(output) => {
                    println!("Compiled Successfully with {prog}! \n {}", output);
                    return Ok(prog_name_stem.to_string());
                }
                Err(err) => {
                    eprintln!("WARNING: Failed to compile code with {prog} with reason {err}");
                }
            };
        }

        eprintln!("Couldn't Compile the code\n");
        Err("Couldn't Compile the code")
    }

    fn run_intrepreted_language(file_path: &Path, lang_name: &Language, stdin_content: &str) -> Result<String, &'static str> {
        let mut program: HashMap<&str, Vec<&str>> = HashMap::new();

        match lang_name {
            Language::Python => {
                program.insert("python3", vec![file_path.to_str().unwrap_or("")]);
                program.insert("python", vec![file_path.to_str().unwrap_or("")]);
            },
            Language::Ruby => {
                program.insert("ruby", vec![file_path.to_str().unwrap_or("")]);
            },
            Language::Javascript => {
                program.insert("node", vec![file_path.to_str().unwrap_or("")]);
                program.insert("deno", vec!["run", file_path.to_str().unwrap_or("")]);
                program.insert("bun", vec![file_path.to_str().unwrap_or("")]);
            },
            _ => {
                return Err("Unsupported/Not a Compiled Language");
            }
        };

        for (prog, args) in &program {
            let std_out = program_utils::run_program_with_input(prog, args, stdin_content);
            match std_out {
                Ok(output) => {
                    println!("Run Successfully with {prog}!");
                    return Ok(output);
                }
                Err(err) => {
                    eprintln!("WARNING: Failed to run code with {prog} with reason {err}");
                }
            };
        }

        Err("Failed to run code!")
    }
}

