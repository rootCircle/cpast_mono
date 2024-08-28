# cpast - Code Testing and Analysis Tool

![Crates.io](https://img.shields.io/crates/d/cpast)
![Crates.io](https://img.shields.io/crates/v/cpast)
![GitHub repo size](https://img.shields.io/github/repo-size/rootCircle/cpast)
![Crates.io](https://img.shields.io/crates/l/cpast)
![docs.rs](https://img.shields.io/docsrs/cpast)
![GitHub Workflow Status (with event)](https://img.shields.io/github/actions/workflow/status/rootCircle/cpast/rust.yml)

> We are working on making cpast, more accessible and simple for all. If you have experience with python and/or writing prompts, consider contributing to [cpast\_llm](https://github.com/rootCircle/cpast_llm) repo.

## Introduction

Have you ever found yourself stuck in the middle of a coding contest, frustrated by a bug or an elusive edge case that just won’t reveal itself? Or maybe you’ve spent countless hours comparing outputs manually, only to find that the real problem was a tricky test case you hadn’t considered. Well, I’ve been there, and I have good news: there’s a solution that can change the way you approach competitive programming forever.

Introducing **cpast**—the game-changing tool designed specifically for competitive programmers and developers who want to enhance their coding efficiency and solve problems faster. Written in Rust for speed and reliability, cpast simplifies the process of comparing outputs from different code files, regardless of the programming language. This allows you to focus on crafting the best solution to your problem without worrying about the technicalities of cross-language compatibility.

Here’s the best part: with cpast, all you need is your _solution file_ and the _input format_, and you can debug your code to perfection without having to look at or rely on anyone else’s code. No more peeking at other people's solutions to figure out what went wrong. Now, you can independently find those tricky cases that make all the difference, allowing you to learn and improve your problem-solving skills honestly and effectively.

> Checkout the blog post [here](https://rootcircle.github.io/blog/project/cpast.html)

## Usecases

0. Debugging your CP/DSA questions.
1. Live hacking in during/post Codeforces contests.
2. Generate testcases for problem setters.
3. Reinforcement learning for large language models and stress testing their code generation accuracy.

## Getting Started

https://github.com/user-attachments/assets/3b7d5b88-5a2a-4d01-8d5b-31f86b9a96db


### Installation

To get started with `cpast`, you need to install it. You can do this by running the following command:

```bash
cargo install cpast
```

<details>
<summary>Installing cargo on Windows</summary>
<br>
On windows, to install cargo, run these commands in terminal (for faster and lighter setup)

```bash
winget install rustup
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu
```

</details>

<details>
<summary>Note for Linux users (clipboard support)</summary>
<br>
On Linux, you'll need to have xorg-dev and libxcb-composite0-dev to compile. On Debian and Ubuntu you can install them with

```bash
sudo apt install xorg-dev libxcb-composite0-dev
```

Required for clipboard support!

Chances are that clipboard support might be broken for some WMs like bspwm, but other features will work just fine!

</details>

<details>
<summary>In case you get a failing build! (for non supported os)</summary>
<br>
cpast is pretty minimalistic by default, but to support basic CLI features like clipboard copying etc, we need to depend on system dependencies, whose support may/may not be provided for unsupported OSes!

We have first class support for macOS, Linux (GNOME, KDE, Hyprland) and Windows

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

* `correct.cpp` should contain the correct code.
* `incorrect.cpp` should contain the incorrect code.
* `(N) (?:N){\1}` is the language generator.
* `100` is the number of test iterations.

#### generate

```bash
cpast generate "S[10,@CH_UPPER@]"
```

* Generates string of length 10, of uppercase characters only

## Language Specification

At the heart of cpast is **clex**, a powerful custom language generator that gives you complete control over input patterns. Imagine regex, but specifically designed for generating random test cases. With clex, you can easily define and automate complex input scenarios to stress-test your code, uncover hidden bugs, and identify edge cases that might otherwise go unnoticed.

For more information on the `clex` language and its usage, please refer to the [Grammar Rules for Clex Generator](./docs/clex/CLEX_LANG_SPECS.md).

## Meta

* [Changelog](./CHANGELOG.md)
* [Future Roadmap](./ROADMAP.md)
* [Clex Language Specs](./docs/clex/CLEX_LANG_SPECS.md)
* [Clex FAQs](./docs/clex/FAQs.md)
* [cpast\_llm](https://github.com/rootCircle/cpast_llm)
* [Alternatives to cpast](./docs/ALTERNATIVES.md)
