use std::path::Path;

use super::runner::{CompilationType, LanguageName};

pub(super) fn get_programming_language_name(file_path: &Path) -> Option<LanguageName> {
    match file_path.extension().and_then(|ext| ext.to_str()) {
        Some("rs") => Some(LanguageName::Rust),
        Some("py") => Some(LanguageName::Python),
        Some("c") => Some(LanguageName::C),
        Some("cpp") | Some("cxx") | Some("c++") | Some("cc") | Some("C") => Some(LanguageName::Cpp),
        Some("java") => Some(LanguageName::Java),
        Some("js") => Some(LanguageName::Javascript),
        Some("rb") => Some(LanguageName::Ruby),
        _ => None,
    }
}

pub(super) fn get_language_compilation_type(lang_name: &LanguageName) -> CompilationType {
    match lang_name {
        LanguageName::Rust | LanguageName::Cpp | LanguageName::C => CompilationType::Compiled,
        LanguageName::Python | LanguageName::Ruby | LanguageName::Javascript => {
            CompilationType::Interpreted
        }
        LanguageName::Java => CompilationType::BytecodeCompiled,
    }
}
