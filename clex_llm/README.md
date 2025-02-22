# clex_llm

> [!NOTE]
> Builds are currently broken for android mobiles, but we have no intention as of now to support it at this moment.

> [!NOTE]
> See previous efforts at  [cpast_llm](https://github.com/rootCircle/cpast_llm).

**clex_llm** is a tool designed to generate [Clex](../clex_gen/README.md) language expressions from input formats and constraints using a language model. It simplifies the process of creating complex test cases by converting human-readable descriptions into formal Clex grammar representations.

## Features

- **Automated Clex Generation**: Automatically generate Clex expressions based on input formats and constraints.
- **Integration with Google Generative AI**: Utilizes Google Generative AI for generating accurate and efficient Clex expressions.

## Getting Started

### Prerequisites

Ensure you have Rust installed on your machine. You can install Rust using [rustup](https://rustup.rs/).

### Installation

Clone the repository and navigate to the `clex_llm` directory:

```bash
git clone https://github.com/rootCircle/cpast_mono.git
cd cpast_mono/clex_llm
```

### Usage

To use `clex_llm`, you need to set up your Google Generative AI API key and run the tests:

```bash
GEMINI_API_KEY="<api-key>" cargo test
```

###

 Example

Here’s a complete example demonstrating how to use the `clex_llm` module:

```rust
use clex_llm::{create_generator, generate_clex_expression};

#[tokio::main]
async fn main() {
    let api_key = "your_google_api_key";
    let generator = create_generator(api_key).unwrap();

    let input_format = "The first line contains an integer K, followed by K lines each containing a floating-point number P.";
    let constraints = "1 ≤ K ≤ 50\n0.0 ≤ P ≤ 1000.0";

    match generate_clex_expression(&generator, input_format, constraints).await {
        Ok(expression) => println!("Generated Clex Expression: {}", expression),
        Err(e) => eprintln!("Error generating Clex expression: {}", e),
    }
}
```
