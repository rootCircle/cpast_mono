//! Example demonstrating time and memory limits in ccode_runner
//!
//! This example shows how to use ExecutionLimits to prevent infinite loops
//! and excessive memory usage when running user code.

use ccode_runner::lang_runner::language_name::LanguageName;
use ccode_runner::lang_runner::program_store::ProgramStore;
use ccode_runner::ExecutionLimits;

fn main() {
    println!("=== ccode_runner Execution Limits Demo ===\n");

    // Example 1: Quick program with limits (should succeed)
    println!("Example 1: Running a quick program with time limit");
    let quick_program = r#"
print("Hello from Python!")
for i in range(5):
    print(f"Count: {i}")
"#;

    let limits = ExecutionLimits::new().with_time_limit(5000); // 5 second timeout

    match ProgramStore::new_from_text_with_limits(
        quick_program,
        quick_program,
        LanguageName::Python,
        LanguageName::Python,
        false,
        limits,
    ) {
        Ok(program) => match program.run_codes_and_compare_output("") {
            Ok((matched, output, _)) => {
                println!("✓ Program executed successfully!");
                println!("  Outputs matched: {}", matched);
                println!("  Output:\n{}", output);
            }
            Err(e) => {
                println!("✗ Execution failed: {}", e);
            }
        },
        Err(e) => {
            println!("✗ Failed to create program: {}", e);
        }
    }

    println!("\n---\n");

    // Example 2: Infinite loop with timeout (should fail)
    println!("Example 2: Running an infinite loop with 1 second timeout");
    let infinite_loop = r#"
while True:
    pass
"#;

    let strict_limits = ExecutionLimits::new().with_time_limit(1000); // 1 second timeout

    match ProgramStore::new_from_text_with_limits(
        infinite_loop,
        infinite_loop,
        LanguageName::Python,
        LanguageName::Python,
        false,
        strict_limits,
    ) {
        Ok(program) => match program.run_codes_and_compare_output("") {
            Ok((matched, output, _)) => {
                println!("✓ Program executed (unexpected!)");
                println!("  Outputs matched: {}", matched);
                println!("  Output: {}", output);
            }
            Err(e) => {
                println!("✓ Program correctly timed out!");
                println!("  Error: {}", e);
            }
        },
        Err(e) => {
            println!("✗ Failed to create program: {}", e);
        }
    }

    println!("\n---\n");

    // Example 3: Program with input and limits
    println!("Example 3: Running a program with stdin and time limits");
    let io_program = r#"
n = int(input())
result = n * n
print(f"Square of {n} is {result}")
"#;

    let io_limits = ExecutionLimits::new().with_time_limit(3000); // 3 second timeout

    match ProgramStore::new_from_text_with_limits(
        io_program,
        io_program,
        LanguageName::Python,
        LanguageName::Python,
        false,
        io_limits,
    ) {
        Ok(program) => match program.run_codes_and_compare_output("42") {
            Ok((matched, output, _)) => {
                println!("✓ Program executed successfully!");
                println!("  Outputs matched: {}", matched);
                println!("  Output: {}", output.trim());
            }
            Err(e) => {
                println!("✗ Execution failed: {}", e);
            }
        },
        Err(e) => {
            println!("✗ Failed to create program: {}", e);
        }
    }

    println!("\n---\n");

    // Example 4: Using both time and memory limits
    println!("Example 4: Program with both time and memory limits");
    let complex_program = r#"
import sys
data = []
for i in range(1000):
    data.append(i * i)
print(f"Computed {len(data)} squares")
print(f"Sum: {sum(data)}")
"#;

    let complex_limits = ExecutionLimits::new()
        .with_time_limit(5000) // 5 seconds
        .with_memory_limit(100 * 1024 * 1024); // 100 MB

    match ProgramStore::new_from_text_with_limits(
        complex_program,
        complex_program,
        LanguageName::Python,
        LanguageName::Python,
        false,
        complex_limits,
    ) {
        Ok(program) => match program.run_codes_and_compare_output("") {
            Ok((matched, output, _)) => {
                println!("✓ Program executed successfully!");
                println!("  Outputs matched: {}", matched);
                println!("  Output:\n{}", output);
            }
            Err(e) => {
                println!("✗ Execution failed: {}", e);
            }
        },
        Err(e) => {
            println!("✗ Failed to create program: {}", e);
        }
    }

    println!("\n=== Demo Complete ===");
}
