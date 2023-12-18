# cpast - Code Testing and Analysis Tool

![Crates.io](https://img.shields.io/crates/d/cpast)
![Crates.io](https://img.shields.io/crates/v/cpast)
![GitHub repo size](https://img.shields.io/github/repo-size/rootCircle/cpast)
![Crates.io](https://img.shields.io/crates/l/cpast)
![docs.rs](https://img.shields.io/docsrs/cpast)
![GitHub Workflow Status (with event)](https://img.shields.io/github/actions/workflow/status/rootCircle/cpast/rust.yml)



`cpast` is a versatile code testing and analysis tool that allows you to test correct and incorrect code files against a custom language generator called `clex`. It supports a variety of programming languages, including Python, C++, C, Rust, Ruby, JavaScript, and Java. You can specify the number of iterations and test your code against random input values, comparing the output against expected results.

## Introduction

`cpast` - Code Testing and Analysis Tool is your solution to a crucial problem faced by competitive programmers (CP) and coding enthusiasts. It empowers you to streamline your coding journey and overcome common challenges in competitive programming and coding practice.

## Addressing a Crucial Problem in Competitive Programming (C.P.)

Competitive programming, often referred to as C.P., involves solving algorithmic and coding challenges within strict time limits. Participants in coding contests, such as ACM ICPC, Codeforces, and LeetCode, often face challenges like:

- Verifying code correctness against various test cases.
- Efficiently testing code under time constraints.
- Debugging errors quickly to improve code performance.

`cpast` has been designed to tackle these challenges head-on and make the competitive programming experience more efficient and enjoyable.

## How `cpast` Solves the Problem

### 1. Testing Correctness

`cpast` enables you to test your code with both correct and incorrect code files, ensuring that your solutions work as expected while also helping you identify and fix issues in your code.

### 2. Rapid Testing

In competitive programming, time is of the essence. `cpast` allows you to define custom test cases and automate testing, saving valuable time that would otherwise be spent manually verifying code correctness.

### 3. Debugging Support

With `cpast`, you can quickly identify and debug issues in your code by comparing actual output with expected results. This helps you fine-tune your code for optimal performance.

By addressing these crucial problems, `cpast` enhances your competitive programming experience, making it more efficient and effective, ultimately improving your coding skills and competition performance. Say goodbye to manual testing and debugging, and let `cpast` handle the heavy lifting for you.

## Table of Contents

- [Features](#features)
- [Getting Started](#getting-started)
- [Language Specification](#language-specification)
- [Example Usage](#example-usage)
- [References](#references)

## Features

- Test correct and incorrect code files.
- Set the number of iterations to run your tests.
- Support for multiple programming languages.

## Getting Started

https://github.com/rootCircle/cpast/assets/35325046/e8f28d06-eba4-4f00-8afc-240dcf7c56f3

### Installation

To get started with `cpast`, you need to install it. You can do this by running the following command:

```bash
cargo install cpast
```

Sidenote: On windows, to install cargo, run these commands in terminal (for faster and lighter setup)

```bash
winget install rustup
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu
```

### Usage

Here's a simple example of how to use `cpast`:

```bash
cpast -c correct.cpp -t incorrect.cpp -g "(N) (?:N){\1}" --iterations 100
```

- `correct.cpp` should contain the correct code.
- `incorrect.cpp` should contain the incorrect code.
- `(N) (?:N){\1}` is the language generator.
- `100` is the number of test iterations.

## Language Specification

The `clex` language generator is based on a custom grammar specification. It allows you to define input patterns for testing. Here are some of the key elements of the `clex` language:

### Meta-characters

- `()?:\{}[],`

### Character Sets

- `SPACE = WHITESPACE | e`
- `N = Integer (-infinity to infinity)`
- `F = Float (-infinity.sth to infinity.sth)`
- `S = Non-whitespace String`
- `C = Non-whitespace Character`

### Special Functions

- `() => Capturing Group Indexed by 1`
- `(?:) => Non-capturing Group`
- `\1 => Back-reference`
- `(?:.....){} => Specify the number of occurrences of the group`
- `N|F[m, n] => Specifying min and max values of N or F (Skip one of the values means MIN and MAX respectively), check for the string if it is within the range or not`

### Language

- `PROGRAM := Vector<PRIMARY_DATA_TYPE | CAPTURING_GROUP | NON_CAPTURING_GROUP>`
- `PRIMARY_DATA_TYPE(REPETITION_STORE) := NUMERAL_TYPE(MIN_VALUE, MAX_VALUE) | CHARACTER | STRING`
- `NUMERAL_TYPE(MIN_VALUE, MAX_VALUE) := INTEGER | FLOAT`
- `CAPTURING_GROUP := PRIMARY_DATA_TYPE(1)::NUMERAL_TYPE(0|POSITIVE_NUMBER, MAX_VALUE)::INTEGER`
- `NON_CAPTURING_GROUP(REPETITION_STORE) := Vector<PRIMARY_DATA_TYPE | NON_CAPTURING_GROUP | CAPTURING_GROUP>`
- `REPETITION_STORE := BY_GROUP(GROUP_NO) | BY_COUNT(POSITIVE_NUMBER) | NONE`

For more information on the `clex` language and its usage, please refer to the [Grammar Rules for Clex Generator](#references).

## Example Usage

Here are some example usages of the `clex` language:

- `(N) N[,1000] (?:N F S){\1}`: Accepts input like "2 2 2 2.2 ABC2 3 4.5 ASD". It expects two integers (with a range from 0 to 1000), followed by triplets of Integer, Float, and String, occurring as many times as specified by the first capturing group.

- `(N[,1000]){\2}`: Valid usage.

- `(?:N[,1000]{\2})`: Valid usage.

- `(?:N{\2}[,1000])`: Invalid usage.

- `(N F)`: Invalid usage. Capturing group can only contain a single non-negative number.

## TODO Later

- [x] Support for Capturing Group inside Non-capturing group
- [ ] Support strong strings checks like all lowercase, uppercase, alphabets, numbers, alphanumeric
- [x] Allow only one time compilations in future
- [ ] Add docs about `clex` usage. For now try inferring from TEST_LANGUAGE.md file.
- [ ] Floating Limit support in Range Bounds for Numeral Data Type for Float
- [x] Support Back-references in Range Bounds as well. 

## References

For more details on the `clex` language and advanced usage, you can refer to the following references:

- [Back-references in repetition construct regex](https://stackoverflow.com/questions/3407696/using-a-regex-back-reference-in-a-repetition-construct-n)
- [Back-references Stack Overflow](https://stackoverflow.com/questions/29728622/regex-with-backreference-as-repetition-count)
- [Possible solution using Code Call-out](https://stackoverflow.com/questions/29728622/regex-with-backreference-as-repetition-count/61898415#61898415)

Now you are ready to use `cpast` for testing your code against various programming languages and input patterns defined by the `clex` language. Happy testing!
