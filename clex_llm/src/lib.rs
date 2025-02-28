//! A crate for generating [Clex](https://github.com/rootCircle/cpast_mono/tree/main/clex_gen) language expressions from input formats and constraints using Google's Generative AI.
//!
//! # Overview
//! This crate provides functionality to convert human-readable input formats and constraints into formal Clex grammar
//! representations using Google's Generative AI model. It helps automate the process of creating test case generators
//! for competitive programming problems.
//!
//! # Examples
//! ```rust
//! use clex_llm::{create_generator, generate_clex_expression};
//!
//! #[tokio::main]
//! async fn main() {
//!     let api_key = "your_google_api_key";
//!     let generator = create_generator(api_key).unwrap();
//!
//!     let input_format = "The first line contains an integer K, followed by K lines each containing a floating-point number P.";
//!     let constraints = "1 ≤ K ≤ 50\n0.0 ≤ P ≤ 1000.0";
//!
//!     match generate_clex_expression(&generator, input_format, constraints).await {
//!         Ok(expression) => println!("Generated Clex Expression: {}", expression),
//!         Err(e) => eprintln!("Error generating Clex expression: {}", e),
//!     }
//! }
//! ```
//!
//! # Features
//! - Generate Clex expressions from natural language descriptions
//! - Integrate with Google's Generative AI
//! - Support for various input formats and constraints
//!
//! # Prerequisites
//! - A valid Google Generative AI API key (get the API key from <https://makersuite.google.com/app/apikey>)
use generator::ClexPromptGenerator;
use google_generative_ai_rs::v1::errors::GoogleAPIError;

mod examples;
mod generator;

/// Creates a new instance of the Clex prompt generator.
///
/// # Arguments
/// * `api_key` - A valid Google Generative AI API key
///
/// # Returns
/// * `Result<ClexPromptGenerator, Box<dyn std::error::Error>>` - A Result containing either the generator instance or an error
///
pub fn create_generator(api_key: &str) -> Result<ClexPromptGenerator, Box<dyn std::error::Error>> {
    generator::ClexPromptGenerator::new(api_key)
}

/// Generates a Clex expression from the given input format and constraints.
///
/// # Arguments
/// * `generator` - A reference to the ClexPromptGenerator instance
/// * `input_format` - A string describing the input format in natural language
/// * `constraints` - A string specifying the constraints on input values
///
/// # Returns
/// * `Result<String, GoogleAPIError>` - A Result containing either the generated Clex expression or a Google API error
///
pub async fn generate_clex_expression(
    generator: &ClexPromptGenerator,
    input_format: &str,
    constraints: &str,
) -> Result<String, GoogleAPIError> {
    generator.generate_response(input_format, constraints).await
}
