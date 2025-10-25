//! Example demonstrating the iterator-based API for memory-efficient test case generation.
//!
//! This example shows how to use the `generator_iter` function to generate large test cases
//! incrementally, which is more memory-efficient than generating the entire test case at once.
//!
//! Run with: `cargo run --example generate_iter --package clex_gen`

use clex_gen::generator_iter;
use std::io::{self, Write};

fn main() {
    // Example 1: Simple usage with small test case
    println!("=== Example 1: Basic Iterator Usage ===");
    let clex = "N[1,10] N[1,10] N[1,10]";
    
    print!("Generated test case: ");
    io::stdout().flush().unwrap();
    
    let mut output = String::new();
    for chunk_result in generator_iter(clex.to_string()).unwrap() {
        match chunk_result {
            Ok(chunk) => {
                print!("{}", chunk);
                io::stdout().flush().unwrap();
                output.push_str(&chunk);
            }
            Err(e) => {
                eprintln!("\nError generating test case: {}", e);
                return;
            }
        }
    }
    
    // Clean up trailing space
    if output.ends_with(' ') {
        output.pop();
    }
    println!();
    println!("Complete output: {}", output);
    println!();

    // Example 2: Streaming to a file (simulated with stdout)
    println!("=== Example 2: Streaming Output ===");
    let clex = "(N[5,5]) (?:N){\\1}";
    
    println!("Generating test case with capturing groups:");
    for (i, chunk_result) in generator_iter(clex.to_string()).unwrap().enumerate() {
        match chunk_result {
            Ok(chunk) => {
                println!("  Chunk {}: {}", i + 1, chunk.trim());
            }
            Err(e) => {
                eprintln!("Error in chunk {}: {}", i + 1, e);
                break;
            }
        }
    }
    println!();

    // Example 3: Large test case scenario
    println!("=== Example 3: Large Test Case (Memory-Efficient) ===");
    // In a real scenario with GiB-sized test cases, the iterator approach
    // allows processing chunks without loading the entire test case into memory
    let clex = "N N N N N N N N N N"; // Simulate multiple large expressions
    
    println!("Processing large test case incrementally:");
    let mut total_size = 0;
    for (i, chunk_result) in generator_iter(clex.to_string()).unwrap().enumerate() {
        match chunk_result {
            Ok(chunk) => {
                total_size += chunk.len();
                println!("  Processed chunk {} ({} bytes)", i + 1, chunk.len());
                // In a real scenario, you might write this to a file or pipe it to another process
                // without accumulating all data in memory
            }
            Err(e) => {
                eprintln!("Error processing chunk {}: {}", i + 1, e);
                break;
            }
        }
    }
    println!("Total output size: {} bytes", total_size);
    println!();

    // Example 4: Error handling
    println!("=== Example 4: Error Handling ===");
    let invalid_clex = "(?:(N)){\\1}"; // This references a group that doesn't exist yet
    
    println!("Attempting to generate with invalid clex:");
    match generator_iter(invalid_clex.to_string()) {
        Ok(iter) => {
            for (i, chunk_result) in iter.enumerate() {
                match chunk_result {
                    Ok(chunk) => println!("  Chunk {}: {}", i + 1, chunk),
                    Err(e) => {
                        eprintln!("  Error in chunk {}: {}", i + 1, e);
                        break;
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("  Error creating iterator: {}", e);
        }
    }
    println!();

    // Example 5: Comparing with traditional approach
    println!("=== Example 5: Memory Efficiency Comparison ===");
    let clex = "S S S"; // Three string expressions with default length
    
    println!("Iterator approach (processes incrementally):");
    let start = std::time::Instant::now();
    let mut iter_size = 0;
    for chunk_result in generator_iter(clex.to_string()).unwrap() {
        if let Ok(chunk) = chunk_result {
            iter_size += chunk.len();
        }
    }
    let iter_duration = start.elapsed();
    println!("  Time: {:?}, Size: {} bytes", iter_duration, iter_size);
    
    println!("Traditional approach (generates all at once):");
    let start = std::time::Instant::now();
    match clex_gen::generator(clex.to_string()) {
        Ok(output) => {
            let trad_duration = start.elapsed();
            println!("  Time: {:?}, Size: {} bytes", trad_duration, output.len());
        }
        Err(e) => eprintln!("  Error: {}", e),
    }
    
    println!();
    println!("Note: The iterator approach is particularly beneficial for:");
    println!("  - Very large test cases (GiB-sized)");
    println!("  - Piping output to another process");
    println!("  - Writing directly to files without memory accumulation");
    println!("  - Streaming test cases to a code runner");
}
