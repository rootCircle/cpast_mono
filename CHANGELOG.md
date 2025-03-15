# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

For changelogs of packages, see:

- [cpast](./cpast/CHANGELOG.md)
- [clex_gen](./clex_gen/CHANGELOG.md)
- [ccode_runner](./ccode_runner/CHANGELOG.md)

## 0.10.0 (2025-03-15)

- BREAKING: Move compilation to temp dir, instead of Path("./")
- BREAKING: Update gemini env key from GEMINI_API_KEY to GOOGLE_API_KEY

### ccode_runner

- Move all compilations to temp dir, instead of Path("./")
- New abstractions `from_text`, `from_custom_dest` and `from_language` for ProgramStore, run the code event from source code text. (Unblocks many cpast_api requests)

### cpast_api

- Implemented all evaluate routes (with_code_and_constraint, with_platform, with_code_and_clex, with_code_and_platform, with_shared_id) (Unblocks cpastcord).
- Caching LLM and scraped responses.
- Inject LLM API key into `Settings` (check cpast_api/configuration/base.yaml)
- Update gemini env key from GEMINI_API_KEY to GOOGLE_API_KEY

### clex_llm

- Update gemini env key from GEMINI_API_KEY to GOOGLE_API_KEY
- Introduce code generation in C++(Unstable)

### cscrapper

- Problem URL parsing support.

## 0.9.2 (2025-03-01)

### clex_gen

- Change default range values to i32_min, i32_max and u32_min and u32_max for positive range.

### cpast_cli

- Improve the completions support.

### general

- Improved crate documentation

## 0.9.1 (2025-02-25)

### clex_gen

- Reduce mutations by segregating states making `generate_testcase` method functional, and removing the need of cloning in case of `cpast_cli`.

### cpast_cli

- hotfix: add semaphore to control max file open errors in cases. Currently defaulted to 100

## 0.9.0 (2025-02-24)

### ccode_runner

- Migrate from sync to async tokio process spawn to improve performance and scalability.

### cpast

- Using updated ccode_runner to improve performance and scalability.

## 0.8.1 (2025-02-23)

### clex_cli

- Add `--clipboard` flag to copy clex to clipboard. Improved `cpast ai` output.

## 0.8.0 (2025-02-23)

### Breaking

- String modifier syntax now accepts three arguments, instead of two. New format: `S[min,max,charset]` (e.g., `S[1,10,'cpast is awesome']`)

### clex_gen

- Now, string modifier accepts escape characters as well like `\n`, `\t`, `\r`, `\\`, `\'`, `\"`, `\0` `\a`, `\b`, `\f` and `\v` etc!
- Breaking: Modified string modifier syntax to support minimum and maximum length constraints:
  - New format: `S[min,max,charset]` (e.g., `S[10,10,@CH_UPPER@]`)
  - Old format: `S[length,charset]` is now deprecated
  - This change enables more precise string length control in pattern generation

### clex_llm

- Switched to Gemini2_0Flash model from Gemini1_0

### cpast_cli

- Support AI based clex generation using input format and constraints.

## 0.7.1 (2025-02-22)

### Overview

Version 0.7.1 marks a significant step forward as cpast transition to a monorepo structure. Building upon the foundation of `cpast_cli`, I'm excited to introduce several new tools to enhance the user experience.

### Breaking Changes

- Removed the `clipboard` feature from `cpast_cli`. (builds might be broken for android[low priority])
- Replaced the `CPAST_DEBUG=1` environment variable with the `--debug` flag in `cpast_cli`.

### Crates

- **ccode_runner**: <https://crates.io/crates/ccode_runner> v0.2.0 (new)
- **clex_gen**: <https://crates.io/crates/clex_gen> v0.2.1 (new)
- **cpast**: <https://crates.io/crates/cpast> v0.7.1 (updated)

### Key Updates

#### Monorepo Migration

- The codebase has been moved to a monorepo, making it easier to manage and allowing for smoother growth in the future.

#### New Tools

- **`cpast_api`**: A Work In Progress (WIP) backend service, currently under development, to facilitate API interactions.
- **`cscrapper`**: A tool for gathering competitive programming questions from CodeChef and Codeforces.
- **`cpastord`**: A service that connects cpast features with Discord.
- **`ccode_runner`**: A flexible engine for running and testing code snippets for various languages. Optimized for repeated runs, robustness, and speed.
- **`clex_gen`**: The core of cpast, this tool generates random test cases using the Clex language. Renamed from `clex` to avoid potential naming conflicts.
- **`clex_llm`**: An AI-powered tool leveraging Google Gemini to generate Clex expressions from natural language.

#### CLI Improvements

- **Verbose Output:** The `cpast_cli` now supports a `--debug` flag for detailed output, replacing the `CPAST_DEBUG` environment variable (See issue #5).
- **Piping in generate:** Improved piping capabilities within the `cpast generate` command.

#### Performance and Security

- **Enhanced Storage Performance:** Optimized the `file_store` component of `ccode_runner` for improved efficiency.
- **Security Audit:** Implemented a security audit workflow using `cargo deny` to scan dependencies for vulnerabilities, ensuring a more secure codebase.

#### Development and Infrastructure

- **Rust 2024 Upgrade:** Updated the codebase to Rust 2024, enabling the use of the latest language features and improvements.
- **Refined Code Structure:** Restructured `cpast_cli` into `ccode_runner` and `clex_gen` for better code organization and maintainability.
- **MSRV:** Set the Minimum Supported Rust Version (MSRV) to 1.85.0.

#### General Improvements

- Improved error propagation across the project.
- Enhanced code quality, documentation, and performance.
- Fixed various bugs, including race conditions and panics.
- Updated dependencies to their latest versions.

For a full list of changes, check out the [Full Changelog](https://github.com/rootCircle/cpast_mono/commits/cpast-v0.7.1).
