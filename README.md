# cpast - Code Testing and Analysis Tool

![Crates.io](https://img.shields.io/crates/d/cpast)
![Crates.io](https://img.shields.io/crates/v/cpast)
![GitHub repo size](https://img.shields.io/github/repo-size/rootCircle/cpast)
![Crates.io](https://img.shields.io/crates/l/cpast)
![docs.rs](https://img.shields.io/docsrs/cpast)
![GitHub Workflow Status (with event)](https://img.shields.io/github/actions/workflow/status/rootCircle/cpast/rust.yml)

> We are working on making cpast, more accesssible and simple for all. If you have experience with python and/or writing prompts, consider contributing to [cpast_llm](https://github.com/rootCircle/cpast_llm) repo.

`cpast` is a versatile code testing and analysis tool that allows you to test correct and incorrect code files against a custom language generator called `clex`. It supports a variety of programming languages, including Python, C++, C, Rust, Ruby, JavaScript, and Java. You can specify the number of iterations and test your code against random input values, comparing the output against expected results.

## Introduction

`cpast` - Code Testing and Analysis Tool is your solution to a crucial problem faced by competitive programmers (CP) and coding enthusiasts. It empowers you to streamline your coding journey and overcome common challenges in competitive programming and coding practice.
Checkout the blog post [here](https://rootcircle.github.io/blog/project/cpast.html)

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

## Usecases

0. Debugging your CP/DSA questions.
1. Live hacking in during/post Codeforces contests.
2. Generate testcases for your problem setters.

## Features

- Test correct and incorrect code files.
- Set the number of iterations to run your tests.
- Support for multiple programming languages.
- [What's new?](./CHANGELOG.md)

## Getting Started

https://github.com/rootCircle/cpast/assets/35325046/1229ce29-f142-4e7a-9008-10b280fb57b6

### Installation

To get started with `cpast`, you need to install it. You can do this by running the following command:

```bash
cargo install cpast
```

<details>
<summary>Note for Windows users</summary>
<br>
On windows, to install cargo, run these commands in terminal (for faster and lighter setup)

```bash
winget install rustup
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu
```

</details>

<details>
<summary>Note for Linux users</summary>
<br>
On Linux, you'll need to have xorg-dev and libxcb-composite0-dev to compile. On Debian and Ubuntu you can install them with

```bash
sudo apt install xorg-dev libxcb-composite0-dev
```

Required for clipboard support,

Chances are that clipboard support might be broken for some WMs like bspwm, but other features will work just fine!
</details>

<details>
<summary>Note for users not running windows, linux or macos; basically android users etc</summary>
<br>

Default compilations won't be supported due to lack of clipboard API support in those systems, and hence you need to compile it with `--no-default-features` feature

```bash
cargo install cpast --no-default-features
```

</details>

### Usage

Here's a simple example of how to use `cpast`:

#### test

```bash
cpast test -c correct.cpp -t incorrect.cpp -g "(N) (?:N){\1}" --iterations 100
```

- `correct.cpp` should contain the correct code.
- `incorrect.cpp` should contain the incorrect code.
- `(N) (?:N){\1}` is the language generator.
- `100` is the number of test iterations.

#### generate

```bash
cpast generate "S[10,'U']"
```

- Generates string of length 10, of uppercase characters only

## Language Specification

The `clex` language generator is based on a custom grammar specification. It allows you to define input patterns for testing.
For more information on the `clex` language and its usage, please refer to the [Grammar Rules for Clex Generator](./CLEX_LANGUAGE.md).

## Roadmap

- [x] Support for Capturing Group inside Non-capturing group
- [x] Support strong strings checks like all lowercase, uppercase, alphabets, numbers, alphanumeric
- [x] Allow only one time compilations in future
- [ ] Support for lazy evaluations of group values.
- [ ] Add docs about `clex` usage. For now try inferring from CLEX_LANGUAGE.md file.
- [ ] Floating Limit support in Range Bounds for Numeral Data Type for Float
- [x] Support Back-references in Range Bounds as well.

Now you are ready to use `cpast` for testing your code against various programming languages and input patterns defined by the `clex` language. Happy testing!

## Alternatives

- Warning: User discretion is required! I don't take any responsibility for any issues faced, while using these alternatives. They are just for informational purposes only. I have not tested either of these.

- Dynamic Testcase Generators
    - [CP-test](https://github.com/ccd97/CP-test)
    - [contest-testcase-generator](https://github.com/tmt514/contest-testcase-generator)
    - [metagen](https://github.com/mingaleg/metagen)
    - [TestCaseGenerator](https://github.com/masterashu/TestCaseGenerator)
    - [testcase-gen](https://github.com/JacobLinCool/testcase-gen)
    - [pycontest](https://github.com/matinhimself/pycontest)
    - [testcase-generator](https://github.com/naskya/testcase-generator)
    - [testcase-generator](https://github.com/Ninjaclasher/testcase-generator)
    - [python-testcase-generator](https://github.com/tjkendev/python-testcase-generator)
    - [inzva-testcase-generator](https://github.com/brkdnmz/inzva-testcase-generator)
    - [MochaGen](https://github.com/CKEFGISC/MochaGen/)
    - [TestcaseGenerator](https://github.com/phirasit/TestcaseGenerator)
    - [genlib](https://github.com/MasterIceZ/genlib)
    - [tc-lexer](https://github.com/bluebottlewize/tc-lexer)
    - [tc-generator](https://github.com/rdxxer/tc-generator)
    - [testcase_generator](https://github.com/hyeonseok92/testcase_generator)
    - [Random_Testcases_Generator](https://github.com/dasilvaca/Random_Testcases_Generator)
    - [python-testcase-generator](https://github.com/tuanpauet/python-testcase-generator)
    - [TestCasesGenerator](https://github.com/khaled-hamam/TestCasesGenerator)
    - [GenerateTestCase](https://github.com/Gingmzmzx/GenerateTestCase)

- Static Testcase Generator
    - [ojtest](https://github.com/f-fanfan/ojtest)
    - [CHD2-12-09-2019](https://github.com/fextivity/CHD2-12-09-2019)

- GPT Based Generators
    - [GPT-testCase-Generator](https://github.com/voho0000/GPT-testCase-Generator)
    - [testcase-generator](https://github.com/DragonBuilder/testcase-generator)

- Misc
    - [tokilib](https://github.com/fushar/tokilib)
    - [testlib](https://github.com/MikeMirzayanov/testlib)
    - [tcframe](https://github.com/ia-toki/tcframe)
    - [tcg](https://github.com/huntzhan/tcg)
    - [TestCase-generator-vscode-extension](https://github.com/yongsk0066/TestCase-generator-vscode-extension)
    - [pnoj-tg](https://github.com/pnoj/pnoj-tg)
    - [TomChienXuTestcaseGenerator](https://github.com/TomChienXuOrganization/TomChienXuTestcaseGenerator)
    - [leetgen](https://github.com/sebnyberg/leetgen)
    - [codeforces-companion](https://github.com/PanagiotisPtr/codeforces-companion)
    - [lc_tcg](https://github.com/deweshsoc/lc_tcg)
    - [code-cross-check](https://github.com/JacobLinCool/code-cross-check)

- Website
    - [testcase-generator](https://github.com/xxxzc/testcase-generator)

