use std::path::Path;

use crate::utils::program_utils;

#[derive(Debug)]
pub(crate) enum Language {
    Python,
    Cpp,
    C,
    Rust,
}

enum CompilationType {
    AheadOfTime, // Compiled language like C, C++, Rust etc
    Jit,         // Java, Go etc
    InTime,      // Python etc
}

impl Language {
    pub fn get_programming_language(file_path: &Path) -> Option<Language> {
        match file_path.extension() {
            Some(src_str) => match src_str.to_str() {
                Some("rs") => Some(Language::Rust),
                Some("py") => Some(Language::Python),
                Some("c") => Some(Language::C),
                Some("cpp") => Some(Language::Cpp),
                _ => None,
            },
            None => None,
        }
    }

    fn get_language_type(lang_type: Language) -> CompilationType {
        match lang_type {
            Language::Rust | Language::Cpp | Language::C => CompilationType::AheadOfTime,
            Language::Python => CompilationType::InTime,
        }
    }

    /// Running single filed self executable program
    pub fn run_program(file_path: &Path) -> Result<String, &str> {
        let lang_name = Self::get_programming_language(file_path);
        match lang_name {
            Some(lang) => {
                let lang_type = Self::get_language_type(lang);
                match lang_type {
                    CompilationType::AheadOfTime => {
                        // Need to Compile and then run
                    }
                    CompilationType::InTime => {
                        // Need to Just Run
                    }
                    CompilationType::Jit => {
                        // Might require converting to intermediate before running (eg java)
                    }
                }
            }
            None => {
                eprintln!("Unsupported Language {}", file_path.to_string_lossy());
                eprintln!("Component: language::Language::detect_program_language");
                std::process::exit(1);
            }
        }
        Ok(String::new())
    }
}

// C => gcc filename -o file_stem
// C => clang filename -o file_stem
// C++ => g++ filename -o file_stem
// C++ => clang++ filename -o file_stem
// Rust => rustc filename -o file_stem
// Rust => cargo run
// Python => python3 filename
// Python => python filename
