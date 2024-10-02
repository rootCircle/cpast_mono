# cpast - Code Testing and Analysis Tool

![Crates.io](https://img.shields.io/crates/d/cpast)
![Crates.io](https://img.shields.io/crates/v/cpast)
![GitHub repo size](https://img.shields.io/github/repo-size/rootCircle/cpast)
![Crates.io](https://img.shields.io/crates/l/cpast)
![docs.rs](https://img.shields.io/docsrs/cpast)

## Introduction

**cpast** is a game-changing tool designed specifically for competitive programmers and developers who want to enhance their coding efficiency. Written in Rust for speed and reliability, cpast simplifies the process of comparing outputs from different code files, allowing you to focus on solving problems effectively.

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

For more information on the `clex` language and its usage, please refer to the [Grammar Rules for Clex Generator](../clex/docs/CLEX_LANG_SPECS.md).

## Meta

* [Changelog](./CHANGELOG.md)
* [Future Roadmap](./docs/ROADMAP.md)
* [Alternatives to cpast](./docs/ALTERNATIVES.md)
