//! Example demonstrating timeout on infinite loops
//!
//! This example shows how execution limits prevent infinite loops
//! by terminating programs that exceed the time limit.

use ccode_runner::ExecutionLimits;
use ccode_runner::lang_runner::language_name::LanguageName;
use ccode_runner::lang_runner::program_store::ProgramStore;

fn main() {
    println!("=== Timeout on Infinite Loop Example ===\n");

    // Infinite loop with timeout (should fail)
    println!("Running an infinite loop with 1 second timeout");
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

    println!("\n=== Example Complete ===");
}
