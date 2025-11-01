//! Example demonstrating combined time and memory limits
//!
//! This example shows how to use both time and memory limits
//! together to control resource usage.

use ccode_runner::ExecutionLimits;
use ccode_runner::lang_runner::language_name::LanguageName;
use ccode_runner::lang_runner::program_store::ProgramStore;

fn main() {
    println!("=== Combined Time and Memory Limits Example ===\n");

    // Using both time and memory limits
    println!("Program with both time and memory limits");
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

    println!("\n=== Example Complete ===");
}
