use std::path::Path;

#[derive(Debug)]
pub(crate) struct ProgramStore<'a> {
    source_file: &'a Path,
    comparing_file: &'a Path,
}

impl<'a> ProgramStore<'a> {
    pub fn new(source_file: &'a Path, comparing_file: &'a Path) -> ProgramStore<'a> {
        Self {
            source_file,
            comparing_file,
        }
    }

    // fn run_program_code(
    //     file_path: &Path,
    //     lang_type: Option<Language>,
    //     stdin_str: String,
    // ) -> Result<String, &str> {
    //     match lang_type {
    //         Some(Language::C) => Command::new("gcc")
    //             .args([
    //                 file_path.to_str().unwrap(),
    //                 "-o",
    //                 file_path.file_stem().unwrap().to_str().unwrap(),
    //             ])
    //             .stdin(),
    //         _ => None,
    //     };
    //     Ok(String::new())
    // }
}
