# cscrapper

**cscrapper** is a tool designed to scrape problem statements from popular competitive programming platforms like CodeChef and CodeForces. It provides a unified interface to fetch problem details, including the problem statement, input format, and constraints.

## Features

- **Multi-Platform Support**: Supports scraping from both CodeChef and CodeForces.
- **Unified Interface**: Provides a single function to fetch problem statements from different platforms.
- **Error Handling**: Gracefully handles errors and provides meaningful error messages.

## Getting Started

### Prerequisites

Ensure you have Rust installed on your machine. You can install Rust using [rustup](https://rustup.rs/).

### Installation

Add `cscrapper` as a dependency in your `Cargo.toml`:

```toml
[dependencies]
cscrapper = "0.1" 
```

### Usage

Here’s a complete example demonstrating how to use the `cscrapper` module:

```rust
use cscrapper::{get_problem_statement, CodePlatform};

fn main() {
    // Fetch a problem statement from CodeChef
    let result = get_problem_statement(CodePlatform::CodeChef("NONNEGPROD"));
    match result {
        Ok(response) => {
            println!("Statement: {}", response.statement);
            println!("Input Format: {}", response.input_format);
            println!("Constraints: {}", response.constraints);
        }
        Err(e) => eprintln!("Error fetching problem statement: {:?}", e),
    }

    // Fetch a problem statement from CodeForces
    let result = get_problem_statement(CodePlatform::CodeForces("1331", "B"));
    match result {
        Ok(response) => {
            println!("Statement: {}", response.statement);
            println!("Input Format: {}", response.input_format);
            println!("Constraints: {}", response.constraints);
        }
        Err(e) => eprintln!("Error fetching problem statement: {:?}", e),
    }
}
```

### Supported Platforms

- **CodeChef**: Fetch problem statements using the problem code.
- **CodeForces**: Fetch problem statements using the contest ID and problem code.
