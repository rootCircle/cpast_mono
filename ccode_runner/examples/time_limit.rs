//! Example demonstrating time limits in ccode_runner
//!
//! This example shows how to use ExecutionLimits to set time limits
//! for running user code to prevent infinite loops.

use ccode_runner::ExecutionLimits;
use ccode_runner::lang_runner::language_name::LanguageName;
use ccode_runner::lang_runner::program_store::ProgramStore;

fn main() {
    println!("=== Time Limit Example ===\n");

    // Quick program with time limit (should succeed)
    println!("Running a quick program with time limit");
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

    println!("\n=== Example Complete ===");
}
