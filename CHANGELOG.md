# Changelog

## 0.4.0

Released on : Jan 30, 2024

- Breaking changes
- AST is reformatted to support new specifications found at clex.specs.md 
- Error Handling done neater
- Bugfix: Fix panic if length of string in StringModifier is negative
- Refactored clex_language
- Support for newline using String using `S[1,'n']`.


## 0.3.4

Released on : Jan 25, 2024

- Multithreading support, improving run times for testcases by more than 45%.
- Refactored the code for more readability and performance
- CLI
  - Colorized outputs

## 0.3.3

Released on : Jan 19, 2024 (Hotfix to 0.3.2)

- Fix error in case if compile binaries are not present by default due to buggy remake implementation.

## 0.3.2

Released on : Jan 19, 2024

- Performance Fixes
  - remake implementation to reduce repeated compilation based on remake implementation in [GNU make](https://www.gnu.org/software/make/)
  - Significant improvement in benchmark performance for `test` with files of compiled programming language. 

## 0.3.1

Released on : Jan 19, 2024 (Hotfix to 0.3.0)

- CLI
  - `test` subcommand now supports an optional `--no-stop` flag, that can be used to never stop after only one failing testcase is found
- Ops
  - This release also address compilation issues of users using android, by using `--no-default-features` flag during compilation.
  - Dependencies update of clap to 4.4.18
- Library
  - compile_and_test method now requires an boolean argument at last to accord to changes in CLI. This argument as addressed earlier too is to whether or not to stop after one failing testcase is found. 


## 0.3.0

Released on : Jan 19, 2024

- CLI
  - Breaking CLI changes, introduction of two subcommands, `test` and `generate`. `test` for running and comparing two files and finding missing edge cases, while `generate` is just to generate the testcase and print it to 
  - `generate` now supports copying testcases over clipboard using `-c` flag, using which you can use testcases in other platforms as well

- Library Changes
  - Strong support for length based checks and charset(string modifiers). Sample usage `cpast generate "S[10,'U']"`
  - 'U' for uppercase, 'L' for lowercase, '0'..'9' for digit, 'N' for Alphanumeric, 'D' for All (including symbols), 'A' for alphabets only!
  - Introduction of support for character literal in cpast, currently being used for string modifier expressions only.
  - Minimum Value for Integer in capturing group now automatically conforms to 0, if negative.
  - Dependencies update
  - Fixed & Updated Docs