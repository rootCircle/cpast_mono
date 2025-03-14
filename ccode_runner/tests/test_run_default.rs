use std::path::Path;

use ccode_runner::lang_runner::program_store::ProgramStore;

#[test]
fn test_run_rust_program() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let file_path = Path::new(&manifest_dir).join("tests/programs/main.rs");

    let program = ProgramStore::new(&file_path, &file_path, false).unwrap();

    let (matched, expected, actual) = program
        .run_codes_and_compare_output("10")
        .expect("Failed to run program");

    assert!(matched);
    assert_eq!(expected, "100\n");
    assert_eq!(actual, "100\n");
}

#[test]
fn test_run_python_program() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let file_path = Path::new(&manifest_dir).join("tests/programs/main.py");

    let program = ProgramStore::new(&file_path, &file_path, false).unwrap();

    let (matched, expected, actual) = program
        .run_codes_and_compare_output("10")
        .expect("Failed to run program");

    assert!(matched);
    assert_eq!(expected, "100.0\n");
    assert_eq!(actual, "100.0\n");
}

#[test]
fn test_run_cpp_program() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let file_path = Path::new(&manifest_dir).join("tests/programs/main.cxx");

    let program = ProgramStore::new(&file_path, &file_path, false).unwrap();

    let (matched, expected, actual) = program
        .run_codes_and_compare_output("10")
        .expect("Failed to run program");

    assert!(matched);
    assert_eq!(expected, "100\n");
    assert_eq!(actual, "100\n");
}

#[test]
fn test_run_c_program() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let file_path = Path::new(&manifest_dir).join("tests/programs/main.c");

    let program = ProgramStore::new(&file_path, &file_path, false).unwrap();

    let (matched, expected, actual) = program
        .run_codes_and_compare_output("10")
        .expect("Failed to run program");

    assert!(matched);
    assert_eq!(expected, "100.00\n");
    assert_eq!(actual, "100.00\n");
}

#[test]
fn test_run_ruby_program() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let file_path = Path::new(&manifest_dir).join("tests/programs/main.rb");

    let program = ProgramStore::new(&file_path, &file_path, false).unwrap();

    let (matched, expected, actual) = program
        .run_codes_and_compare_output("10")
        .expect("Failed to run program");

    assert!(matched);
    assert_eq!(expected, "100.0\n");
    assert_eq!(actual, "100.0\n");
}

#[test]
fn test_run_javascript_program() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let file_path = Path::new(&manifest_dir).join("tests/programs/main.js");

    let program = ProgramStore::new(&file_path, &file_path, false).unwrap();

    // Soundness bug: Readline in JS requires EOL to read from stdin
    let (matched, expected, actual) = program
        .run_codes_and_compare_output("10\n")
        .expect("Failed to run program");

    assert!(matched);
    assert_eq!(expected, "100\n");
    assert_eq!(actual, "100\n");
}

#[test]
fn test_run_java_program() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let file_path = Path::new(&manifest_dir).join("tests/programs/Main.java");

    let program = ProgramStore::new(&file_path, &file_path, false).unwrap();

    let (matched, expected, actual) = program
        .run_codes_and_compare_output("10")
        .expect("Failed to run program");

    assert!(matched);
    assert_eq!(expected, "100.0\n");
    assert_eq!(actual, "100.0\n");
}
