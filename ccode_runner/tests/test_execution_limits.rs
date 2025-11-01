use ccode_runner::ExecutionLimits;
use ccode_runner::lang_runner::{language_name::LanguageName, program_store::ProgramStore};

#[test]
fn test_time_limit_exceeded_infinite_loop_python() {
    let program_text = r#"
while True:
    pass
"#;

    let limits = ExecutionLimits::new().with_time_limit(1000); // 1 second timeout

    let result = ProgramStore::new_from_text_with_limits(
        program_text,
        program_text,
        LanguageName::Python,
        LanguageName::Python,
        false,
        limits,
    );

    assert!(result.is_ok(), "ProgramStore creation should succeed");
    let program = result.unwrap();

    let run_result = program.run_codes_and_compare_output("");
    assert!(run_result.is_err(), "Should fail due to timeout");

    // The error can be either CodeRunFailed (wrapping timeout) or direct timeout
    // Both are acceptable as the timeout is being enforced
    let error = run_result.unwrap_err();
    let error_msg = format!("{}", error);
    assert!(
        error_msg.contains("time limit")
            || error_msg.contains("exceeded")
            || error_msg.contains("CodeRunFailed"),
        "Error should indicate failure, got: {}",
        error_msg
    );
}

#[test]
fn test_time_limit_not_exceeded_quick_program_python() {
    let program_text = r#"
print("Hello, World!")
"#;

    let limits = ExecutionLimits::new().with_time_limit(5000); // 5 second timeout

    let result = ProgramStore::new_from_text_with_limits(
        program_text,
        program_text,
        LanguageName::Python,
        LanguageName::Python,
        false,
        limits,
    );

    assert!(result.is_ok(), "ProgramStore creation should succeed");
    let program = result.unwrap();

    let run_result = program.run_codes_and_compare_output("");
    assert!(run_result.is_ok(), "Should succeed within time limit");

    let (matched, expected, actual) = run_result.unwrap();
    assert!(matched);
    assert_eq!(expected.trim(), "Hello, World!");
    assert_eq!(actual.trim(), "Hello, World!");
}

#[test]
fn test_time_limit_exceeded_infinite_loop_c() {
    let program_text = r#"
int main() {
    while (1) {}
    return 0;
}
"#;

    let limits = ExecutionLimits::new().with_time_limit(1000); // 1 second timeout

    let result = ProgramStore::new_from_text_with_limits(
        program_text,
        program_text,
        LanguageName::C,
        LanguageName::C,
        false,
        limits,
    );

    assert!(result.is_ok(), "ProgramStore creation should succeed");
    let program = result.unwrap();

    let run_result = program.run_codes_and_compare_output("");
    assert!(run_result.is_err(), "Should fail due to timeout");
}

#[test]
fn test_time_limit_not_exceeded_quick_program_c() {
    let program_text = r#"
#include <stdio.h>
int main() {
    printf("Hello, World!\n");
    return 0;
}
"#;

    let limits = ExecutionLimits::new().with_time_limit(5000); // 5 second timeout

    let result = ProgramStore::new_from_text_with_limits(
        program_text,
        program_text,
        LanguageName::C,
        LanguageName::C,
        false,
        limits,
    );

    assert!(result.is_ok(), "ProgramStore creation should succeed");
    let program = result.unwrap();

    let run_result = program.run_codes_and_compare_output("");
    assert!(run_result.is_ok(), "Should succeed within time limit");

    let (matched, expected, actual) = run_result.unwrap();
    assert!(matched);
    assert_eq!(expected.trim(), "Hello, World!");
    assert_eq!(actual.trim(), "Hello, World!");
}

#[test]
fn test_backwards_compatibility_no_limits() {
    let program_text = r#"
print("Backwards compatible!")
"#;

    let program = ProgramStore::new_from_text(
        program_text,
        program_text,
        LanguageName::Python,
        LanguageName::Python,
        false,
    )
    .unwrap();

    let (matched, expected, actual) = program
        .run_codes_and_compare_output("")
        .expect("Failed to run program");

    assert!(matched);
    assert_eq!(expected.trim(), "Backwards compatible!");
    assert_eq!(actual.trim(), "Backwards compatible!");
}

#[test]
fn test_execution_limits_builder_pattern() {
    let limits = ExecutionLimits::new()
        .with_time_limit(2000)
        .with_memory_limit(100 * 1024 * 1024);

    assert_eq!(limits.time_limit_ms, Some(2000));
    assert_eq!(limits.memory_limit_bytes, Some(100 * 1024 * 1024));
}

#[test]
fn test_execution_limits_default() {
    let limits = ExecutionLimits::default();

    assert_eq!(limits.time_limit_ms, None);
    assert_eq!(limits.memory_limit_bytes, None);
}

#[test]
fn test_time_limit_with_stdin() {
    let program_text = r#"
n = int(input())
print(n * n)
"#;

    let limits = ExecutionLimits::new().with_time_limit(5000); // 5 second timeout

    let result = ProgramStore::new_from_text_with_limits(
        program_text,
        program_text,
        LanguageName::Python,
        LanguageName::Python,
        false,
        limits,
    );

    assert!(result.is_ok(), "ProgramStore creation should succeed");
    let program = result.unwrap();

    let run_result = program.run_codes_and_compare_output("10");
    assert!(run_result.is_ok(), "Should succeed with stdin input");

    let (matched, expected, actual) = run_result.unwrap();
    assert!(matched);
    assert_eq!(expected.trim(), "100");
    assert_eq!(actual.trim(), "100");
}

#[test]
fn test_memory_limit_only_no_timeout() {
    // Test program that runs quickly but with memory limit set
    let program_text = r#"
print("Quick program with memory limit")
"#;

    let limits = ExecutionLimits::new().with_memory_limit(100 * 1024 * 1024); // 100MB

    let result = ProgramStore::new_from_text_with_limits(
        program_text,
        program_text,
        LanguageName::Python,
        LanguageName::Python,
        false,
        limits,
    );

    assert!(result.is_ok(), "ProgramStore creation should succeed");
    let program = result.unwrap();

    let run_result = program.run_codes_and_compare_output("");
    assert!(
        run_result.is_ok(),
        "Should succeed with memory limit on quick program"
    );

    let (matched, expected, _) = run_result.unwrap();
    assert!(matched);
    assert_eq!(expected.trim(), "Quick program with memory limit");
}

#[test]
fn test_combined_time_and_memory_limits() {
    // Test with both time and memory limits
    let program_text = r#"
import sys
print("Program with both limits")
"#;

    let limits = ExecutionLimits::new()
        .with_time_limit(5000)
        .with_memory_limit(100 * 1024 * 1024);

    let result = ProgramStore::new_from_text_with_limits(
        program_text,
        program_text,
        LanguageName::Python,
        LanguageName::Python,
        false,
        limits,
    );

    assert!(result.is_ok(), "ProgramStore creation should succeed");
    let program = result.unwrap();

    let run_result = program.run_codes_and_compare_output("");
    assert!(run_result.is_ok(), "Should succeed with both limits");

    let (matched, expected, _) = run_result.unwrap();
    assert!(matched);
    assert_eq!(expected.trim(), "Program with both limits");
}

#[test]
fn test_memory_limit_with_rust() {
    // Test memory limit with a compiled language
    let program_text = r#"
fn main() {
    println!("Rust with memory limit");
}
"#;

    let limits = ExecutionLimits::new()
        .with_time_limit(5000)
        .with_memory_limit(50 * 1024 * 1024); // 50MB

    let result = ProgramStore::new_from_text_with_limits(
        program_text,
        program_text,
        LanguageName::Rust,
        LanguageName::Rust,
        false,
        limits,
    );

    assert!(result.is_ok(), "ProgramStore creation should succeed");
    let program = result.unwrap();

    let run_result = program.run_codes_and_compare_output("");
    assert!(
        run_result.is_ok(),
        "Should succeed with memory limit on Rust"
    );

    let (matched, expected, _) = run_result.unwrap();
    assert!(matched);
    assert_eq!(expected.trim(), "Rust with memory limit");
}

#[test]
fn test_only_time_limit_no_memory() {
    // Test with only time limit, no memory limit
    let program_text = r#"
print("Only time limit")
"#;

    let limits = ExecutionLimits::new().with_time_limit(5000);

    let result = ProgramStore::new_from_text_with_limits(
        program_text,
        program_text,
        LanguageName::Python,
        LanguageName::Python,
        false,
        limits,
    );

    assert!(result.is_ok(), "ProgramStore creation should succeed");
    let program = result.unwrap();

    let run_result = program.run_codes_and_compare_output("");
    assert!(run_result.is_ok(), "Should succeed with time limit only");

    let (matched, expected, _) = run_result.unwrap();
    assert!(matched);
    assert_eq!(expected.trim(), "Only time limit");
}

#[test]
fn test_cpp_with_limits() {
    // Test with C++ to cover compiled language paths
    let program_text = r#"
#include <iostream>
int main() {
    std::cout << "C++ with limits" << std::endl;
    return 0;
}
"#;

    let limits = ExecutionLimits::new()
        .with_time_limit(5000)
        .with_memory_limit(50 * 1024 * 1024);

    let result = ProgramStore::new_from_text_with_limits(
        program_text,
        program_text,
        LanguageName::Cpp,
        LanguageName::Cpp,
        false,
        limits,
    );

    assert!(result.is_ok(), "ProgramStore creation should succeed");
    let program = result.unwrap();

    let run_result = program.run_codes_and_compare_output("");
    assert!(run_result.is_ok(), "Should succeed with limits on C++");

    let (matched, expected, _) = run_result.unwrap();
    assert!(matched);
    assert_eq!(expected.trim(), "C++ with limits");
}
