//! # ccode_runner
//!
//! `ccode_runner` is a crate designed to execute code snippets in various programming languages.
//! It supports both compiled and interpreted languages, providing a unified interface for running code and comparing outputs.
//!
//! ## Features
//!
//! - **Multi-language Support**: Capable of running code in Rust, Python, C, C++, Java, Ruby, and JavaScript.
//! - **Compilation and Interpretation**: Automatically handles compilation for languages like C, C++, Rust, and Java, and interpretation for languages like Python, Ruby, and JavaScript.
//! - **Optimized Execution**: Implements caching and precompilation to reduce execution times, especially useful in scenarios with repeated executions.
//! - **Program Store**: Manages and stores compiled programs for efficient reuse.
//! - **Execution Limits**: Support for time and memory limits to prevent runaway processes.
//!
//! ## Modules
//!
//! - `lang_runner`: Contains the core logic for running code in different languages.
//! - `utils`: Provides utility functions used throughout the crate.
//!
pub mod lang_runner;
mod utils;

// Re-export ExecutionLimits for public API
pub use utils::program_utils::ExecutionLimits;
