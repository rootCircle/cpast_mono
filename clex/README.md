# clex - Custom Language Generator

**clex** is a powerful language generator designed specifically for creating random test cases. It allows you to automate the generation of complex input scenarios, enabling developers to stress-test their code effectively.

## Features

- **Customizable Patterns**: Define and automate input patterns using a specialized syntax.
- **Integration with cpast**: Works seamlessly with the cpast tool for enhanced debugging and testing.

## Getting Started

The best way to realize clex would be to use cpast CLI, instructions to setup it up is given in [cpast README](../cpast/README.md) or you can use it as a lib as well.

### Prerequisites

Ensure you have Rust installed on your machine. You can install Rust using [rustup](https://rustup.rs/).

### Installation

To install clex, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
clex = "0.1"  // Adjust the version as necessary
```

### Usage

Here’s a complete example demonstrating how to use the functions provided by the `clex` module:

```rust
use clex::{get_tokens, get_ast, generator};

// Get tokens from custom language
let tokens = get_tokens("(N) (?:N){\\1}".to_string()).unwrap();
println!("Tokens: {:?}", tokens);

// Get the Abstract Syntax Tree (AST)
let ast = get_ast("(N) (?:N){\\1}".to_string()).unwrap();
println!("AST: {:?}", ast);

// Generate code based on the custom language specification
let generated_code = generator("(N[1,10]) (?:N){\\1}".to_string()).unwrap();
println!("Generated Code: {}", generated_code);
```

### Clex Language Specification

For more information on the clex language and its usage, refer to the [Clex Language Specs](./docs/CLEX_LANG_SPECS.md).

### Examples

- `N{2}`: Generates two random integers.
- `(N) (?:N){\\1}`: Generates a random integer, then the same number of additional integers.
- `(N) (?:S[\\1,])`: Generates a random integer, then a string of that length.
- `(N) (?:S[\\1,@CH_UPPER@])`: Generates a random integer followed by a random string of uppercase letters, where the length of the string is equal to the generated integer.
- `N S C`: Generates a random integer, string, and character.
- `F[-100,100]`: Generates a random floating-point number between -100 and 100.
- `(N[1,100]) (?:N[1,1000]){\\1} N[1,10000]`: Captures a random integer between 1 and 100, then generates that many integers between 1 and 1000, followed by another integer between 1 and 10000.

## Meta

- [Language Specification](./docs/CLEX_LANG_SPECS.md)
- [Clex FAQs](./docs/CLEX_LANG_FAQs.md)
