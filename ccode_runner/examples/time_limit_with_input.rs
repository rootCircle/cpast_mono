//! Example demonstrating time limits with stdin input
//!
//! This example shows how to use time limits on programs
//! that require stdin input.

use ccode_runner::ExecutionLimits;
use ccode_runner::lang_runner::language_name::LanguageName;
use ccode_runner::lang_runner::program_store::ProgramStore;

fn main() {
    println!("=== Time Limit with Input Example ===\n");

    // Program with input and limits
    println!("Running a program with stdin and time limits");
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

    println!("\n=== Example Complete ===");
}
