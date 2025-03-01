# cpast_cli - Code Testing and Analysis Tool

![Crates.io](https://img.shields.io/crates/v/cpast?style=for-the-badge&logo=rust&logoColor=D9E0EE&labelColor=292324)
![Crates.io](https://img.shields.io/crates/d/cpast?style=for-the-badge&logo=rust&logoColor=D9E0EE&labelColor=292324)
![Crates.io](https://img.shields.io/crates/l/cpast?style=for-the-badge&logo=rust&logoColor=D9E0EE&labelColor=292324)
![docs.rs](https://img.shields.io/docsrs/cpast?style=for-the-badge&logo=rust&logoColor=D9E0EE&labelColor=292324)
![Crates.io Size](https://img.shields.io/crates/size/cpast?style=for-the-badge&logo=rust&logoColor=D9E0EE&labelColor=292324)
![Crates.io MSRV](https://img.shields.io/crates/msrv/cpast?style=for-the-badge&logo=rust&logoColor=D9E0EE&labelColor=292324)

## Introduction

> [!WARNING]
> Builds are currently broken for android mobiles as we recently removed clipboard support feature! We are thinking more for the fix along the way

**cpast_cli**(from now on cpast) is a game-changing _CLI_ tool designed specifically for competitive programmers and developers who want to enhance their coding efficiency. Written in Rust for speed and reliability, cpast simplifies the process of comparing outputs from different code files, allowing you to focus on solving problems effectively.

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

```bash
cargo install cpast 
```

</details>

### Usage

Here's a simple example of how to use `cpast`:

#### test

```bash
cpast test -c correct.cpp -t incorrect.cpp -g "(N) (?:N){\\1}" --iterations 100
```

* `correct.cpp` should contain the correct code.
* `incorrect.cpp` should contain the incorrect code.
* `(N) (?:N){\1}` is the language generator.
* `100` is the number of test iterations.

`cpast test --debug ...` can be used to debug the test cases generated.

#### generate

```bash
cpast generate "S[10,10,@CH_UPPER@]"
```

* Generates string of length 10, of uppercase characters only

### ai

```bash
GEMINI_API_KEY="<gemini-api-key>" cpast ai --input-format="The first line contains an integer T (number of test cases). Each of the next T lines contains two integers N and M." --constraints="1 ≤ T ≤ 10\n1 ≤ N, M ≤ 10^5"
```

* Generates clex using LLM from input format and constraints.
* Get API key from <https://makersuite.google.com/app/apikey>
* Alternatively, expose the API key from the shell configuration file like bashrc, zshrc etc using:

  ```bash
  export GEMINI_API_KEY='<api key here>';
  ```

### Shell Completions

To generate shell completions for `cpast`, use the following commands based on your shell:

* **zsh**:

  ```zsh
  cpast --completions=zsh | sudo tee /usr/local/share/zsh/site-functions/_cpast
  ```

* **fish**:

  ```fish
  cpast --completions=fish > ~/.local/share/fish/generated_completions/cpast.fish
  ```

* **bash**:

  ```bash
  cpast --completions=bash | sudo tee /etc/bash_completion.d/cpast.bash
  ```

## Language Specification

At the heart of cpast is **clex_gen**, a powerful custom language generator that gives you complete control over input patterns. Imagine regex, but specifically designed for generating random test cases. With clex, you can easily define and automate complex input scenarios to stress-test your code, uncover hidden bugs, and identify edge cases that might otherwise go unnoticed.

For more information on the `clex` language and its usage, please refer to the [Grammar Rules for Clex Generator](../clex_gen/docs/CLEX_LANG_SPECS.md).

## Backslash Plague in CLI

When using cpast in the CLI, you'll need to properly escape special characters in clex expressions, particularly backslashes in repetition patterns. For example:

```bash
# Incorrect usage
cpast test -g "(N) N{\1}"

# Correct usage with escaped backslash
cpast test -g "(N) N{\\1}"
```

## Meta

* [Changelog](./CHANGELOG.md)
* [Future Roadmap](./docs/ROADMAP.md)
* [Alternatives to cpast](./docs/ALTERNATIVES.md)
