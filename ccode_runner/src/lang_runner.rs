//! This module provides functionality for running and managing code in various programming languages.
//!
//! # Modules
//!
//! * `file_store` - Internal interface and module for managing source code files
//! * `language_name` - Public module containing language name definitions and utilities
//! * `program_store` - Public module for storing and managing program source code with caching support, built on top of file_store.
//! * `runner` - Internal module implementing core code execution functionality
//! * `runner_error_types` - Public module defining error types that can occur during code execution

pub(crate) mod file_store;
pub mod language_name;
pub mod program_store;
pub mod runner;
pub mod runner_error_types;
