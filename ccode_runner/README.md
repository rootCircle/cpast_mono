# ccode_runner

![Crates.io](https://img.shields.io/crates/v/ccode_runner?style=for-the-badge&logo=rust&logoColor=D9E0EE&labelColor=292324)
![Crates.io](https://img.shields.io/crates/d/ccode_runner?style=for-the-badge&logo=rust&logoColor=D9E0EE&labelColor=292324)
![Crates.io](https://img.shields.io/crates/l/ccode_runner?style=for-the-badge&logo=rust&logoColor=D9E0EE&labelColor=292324)
![docs.rs](https://img.shields.io/docsrs/ccode_runner?style=for-the-badge&logo=rust&logoColor=D9E0EE&labelColor=292324)
![Crates.io Size](https://img.shields.io/crates/size/ccode_runner?style=for-the-badge&logo=rust&logoColor=D9E0EE&labelColor=292324)
![Crates.io MSRV](https://img.shields.io/crates/msrv/ccode_runner?style=for-the-badge&logo=rust&logoColor=D9E0EE&labelColor=292324)

`ccode_runner` is a component designed to run arbitrary program code on local devices. It compiles or interprets code and sends the output, making it an essential part of the cpast ecosystem.

## Features

- **Multi-language Support**: Supports various programming languages including Rust, Python, C, C++, Java, Ruby, and JavaScript.
- **Compilation and Interpretation**: Handles both ahead-of-time compilation and just-in-time interpretation.
- **Optimized Execution**: Uses precompilation and caching to optimize execution times.
- **Execution Limits**: Configure time and memory limits to prevent runaway processes and excessive resource consumption.

## Getting Started

### Prerequisites

Ensure you have the necessary compilers and interpreters installed for the languages you intend to use.

### Installation

Clone the repository and navigate to the `ccode_runner` directory:

```bash
git clone https://github.com/rootCircle/cpast_mono.git
cd cpast_mono/ccode_runner
```

### Usage

To use `ccode_runner`, you need to integrate it within your cpast testing workflow. Below is an example of how to use it:

```rust
use ccode_runner::lang_runner::program_store::ProgramStore;
use std::path::Path;

fn main() {
    let correct_file = Path::new("path/to/correct_file.rs");
    let test_file = Path::new("path/to/test_file.rs");
    let do_force_compile = true;

    let program_store = ProgramStore::new(correct_file, test_file, do_force_compile).unwrap();

    let stdin_content = "input data";
    let (is_different, correct_output, test_output) = program_store
        .run_codes_and_compare_output(stdin_content)
        .unwrap();

    println!("Outputs are different: {}", is_different);
    println!("Correct Output: {}", correct_output);
    println!("Test Output: {}", test_output);
}
```

### Using Execution Limits

You can configure time and memory limits to prevent infinite loops and excessive resource consumption:

**Platform Support:**
- **Time limits**: Supported on all platforms (Unix, Windows, macOS)
- **Memory limits**: Supported on all platforms
  - Unix/Linux/macOS: Native OS enforcement via `setrlimit(RLIMIT_AS)`
  - Windows: Active monitoring and enforcement via process memory tracking

```rust
use ccode_runner::lang_runner::program_store::ProgramStore;
use ccode_runner::ExecutionLimits;
use std::path::Path;

fn main() {
    let correct_file = Path::new("path/to/correct_file.rs");
    let test_file = Path::new("path/to/test_file.rs");
    let do_force_compile = true;
    
    // Configure limits: 5 second timeout and 512MB memory limit
    let limits = ExecutionLimits::new()
        .with_time_limit(5000)  // 5000 milliseconds
        .with_memory_limit(512 * 1024 * 1024);  // 512 MB
    
    let program_store = ProgramStore::new_with_limits(
        correct_file, 
        test_file, 
        do_force_compile,
        limits
    ).unwrap();

    let stdin_content = "input data";
    let result = program_store.run_codes_and_compare_output(stdin_content);
    
    match result {
        Ok((is_different, correct_output, test_output)) => {
            println!("Outputs are different: {}", is_different);
            println!("Correct Output: {}", correct_output);
            println!("Test Output: {}", test_output);
        }
        Err(e) => {
            eprintln!("Execution failed: {}", e);
        }
    }
}
```

### Supported Languages

- **Rust**: `.rs`
- **Python**: `.py`
- **C**: `.c`
- **C++**: `.cpp`, `.cxx`, `.c++`, `.cc`, `.C`
- **Java**: `.java`
- **JavaScript**: `.js`
- **Ruby**: `.rb`

### Compilation and Execution

`ccode_runner` uses different strategies for different languages:

- **Ahead-of-Time Compilation**: For languages like C, C++, Rust, and Java.
- **Just-in-Time Interpretation**: For languages like Python, Ruby, and JavaScript.
- **Ahead-of-Time Interpreted**: For Java, which requires converting to intermediate bytecode before execution.

ccode_runner is well suited when repeated compilation might be required for one code like in case for cpast, it intelligently skips those cases for you, making it lot faster!

## Contributing

We welcome contributions! Please read our [Contributing Guidelines](../CONTRIBUTING.md) for more details.
