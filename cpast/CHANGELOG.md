# Changelog

## 0.8.0 (2025-02-23)

<csr-id-92ee0a4ff8513df123f3a67844cd048d607be264/>

### Chore

 - <csr-id-92ee0a4ff8513df123f3a67844cd048d607be264/> add shell completion instructions

### Documentation

 - <csr-id-0d6e667a7a3087106e377efd5c2c96881f63caa7/> improved shields badges and README

### New Features

 - <csr-id-ebc1e6e3d20e2fdee80ba6c0cb780f2c0d4db06e/> introduce ai subcommand to generate clex form input format and constraints
 - <csr-id-1fa604a848dc960908c7148efde4621a38f2a573/> add support for min,max length in string as well espace characters in custom charsets and updated llm model to gemini 2_0flash
 - <csr-id-bc8f08ba637c113645a417d558e149dbe16bdd3a/> introduce ai subcommand to generate clex form input format and constraints
 - <csr-id-7fbe621497981a93f390f1857537b540420c8d18/> introduce ai subcommand to generate clex form input format and constraints
 - <csr-id-d7f610314c38fdad56d297a1371a72e343085212/> introduce ai subcommand to generate clex form input format and constraints
 - <csr-id-f8cb2b04dabe45c08ba94f7e7bdc68eaa8c1755e/> add --clipboard for cpast ai

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release clex_llm v0.2.0, cpast v0.8.0 ([`d6131c8`](https://github.com/rootCircle/cpast_mono/commit/d6131c80a0c84a000a1787a10301c74390cce088))
    - Add --clipboard for cpast ai ([`f8cb2b0`](https://github.com/rootCircle/cpast_mono/commit/f8cb2b04dabe45c08ba94f7e7bdc68eaa8c1755e))
</details>

## 0.7.1 (2025-02-22)

<csr-id-be396e164ba64e5cda157ce5ad4d93fe503bf976/>
<csr-id-1a1d5113a4797c98ce15f4c8467e3807806139a8/>
<csr-id-a898f15e10691ff4d2389ba9baa1eebaf81c6421/>
<csr-id-bd06417f6935b916ab6647ddbb40880fd9388c7d/>
<csr-id-03199312b8347f21e93b44dab9cbce3c538182f1/>
<csr-id-8a000e047deebefdbe34b6c52656c342f149f099/>
<csr-id-e98a8df53a173d3a51ec2a30cf126802793d0990/>
<csr-id-139c68a9a1f7178749e6297875fd01437d8b4ac4/>
<csr-id-0a04f6f80d8f1c544aeee6fad96a8c366dd2b9ca/>
<csr-id-05cc73bdc5887c67fdf86d02df60fc90e2109683/>
<csr-id-c4e4fe1a126481c850a4f78bd4011125ff988b06/>
<csr-id-27ba21a737a84005359317eb48cca12100405b32/>
<csr-id-9a63c718ab3848503f75ff7e9bb1b5fbc022021b/>

### Chore

 - <csr-id-be396e164ba64e5cda157ce5ad4d93fe503bf976/> catch os 32 broken pipe errors
 - <csr-id-1a1d5113a4797c98ce15f4c8467e3807806139a8/> ignore other program file used for testing
 - <csr-id-a898f15e10691ff4d2389ba9baa1eebaf81c6421/> add cpastcord
 - <csr-id-bd06417f6935b916ab6647ddbb40880fd9388c7d/> new release
 - <csr-id-03199312b8347f21e93b44dab9cbce3c538182f1/> restructing folder structure cpast

### Refactor

 - <csr-id-9a63c718ab3848503f75ff7e9bb1b5fbc022021b/> rename clex package to clex_gen

### Documentation

 - <csr-id-e670b0ca127f2755ea7ad090f0283cc2bf4cdbc7/> modified/add README for better segregation

### New Features

 - <csr-id-0d1e7e089c9f682a95918feddd139a0e33f9d67a/> improves error types
 - <csr-id-18f1c5182c4fd105242aeb7f851edbbeafd778d7/> introduce --debug flag (closes #5)
 - <csr-id-a29a4c1da0732dbf2e9cf3f86873a635b7896592/> new file store interface and mig to rust 2024
 - <csr-id-ebec48d4366044ecc318cd99afb67dc4f16613ee/> remove clipboard feature
 - <csr-id-61afca4da7d3df0e59fb9ac8b018a476fd1707f2/> refactor cpast into ccode refactor and cli
 - <csr-id-6d491f5355fb74a14cd556d6d777a070bbb1f007/> move to monorepo

### Bug Fixes

 - <csr-id-94458dd0ae5d90e46b6871a7b6eca43ec472d107/> allow pipe of generated testcases easily

### Other

 - <csr-id-8a000e047deebefdbe34b6c52656c342f149f099/> cpast_cli clex ccode_runner
 - <csr-id-e98a8df53a173d3a51ec2a30cf126802793d0990/> rename to cpast_mono from cpast
 - <csr-id-139c68a9a1f7178749e6297875fd01437d8b4ac4/> update msrv
 - <csr-id-0a04f6f80d8f1c544aeee6fad96a8c366dd2b9ca/> use worspace deps
 - <csr-id-05cc73bdc5887c67fdf86d02df60fc90e2109683/> fix failing clipboard test pipeline
 - <csr-id-c4e4fe1a126481c850a4f78bd4011125ff988b06/> fix failing ci

### Refactor

 - <csr-id-27ba21a737a84005359317eb48cca12100405b32/> use atomicbool instead of mutex in compile_and_test

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 29 commits contributed to the release over the course of 116 calendar days.
 - 21 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release clex_gen v0.2.1, cpast v0.7.1 ([`e50930b`](https://github.com/rootCircle/cpast_mono/commit/e50930bcf32be4bf4a29f6cfea8fb51d72226482))
    - Release clex_gen v0.2.1, cpast v0.7.1 ([`322a597`](https://github.com/rootCircle/cpast_mono/commit/322a5976a09ebb1c49241d08d2b11c07d0d9cd5e))
    - Rename clex package to clex_gen ([`9a63c71`](https://github.com/rootCircle/cpast_mono/commit/9a63c718ab3848503f75ff7e9bb1b5fbc022021b))
    - Release ccode_runner v0.2.0, clex v0.2.1, cpast v0.7.1 ([`7e750cc`](https://github.com/rootCircle/cpast_mono/commit/7e750cc72b592bd491f4f503fc72c19043934f1b))
    - Release ccode_runner v0.2.0, clex v0.2.1, cpast v0.7.1 ([`2cfb445`](https://github.com/rootCircle/cpast_mono/commit/2cfb44521a215d57afe95139a830ed442518e2b8))
    - Release ccode_runner v0.2.0, clex v0.2.1, cpast v0.7.1 ([`447fbef`](https://github.com/rootCircle/cpast_mono/commit/447fbef5fb82b81391a2a8a6e827e3870756f961))
    - Release ccode_runner v0.2.0, clex v0.2.1, cpast v0.7.1 ([`a5478ea`](https://github.com/rootCircle/cpast_mono/commit/a5478ea8c1548147655142d73b6d82e8d7676cb7))
    - Release ccode_runner v0.2.0, clex v0.2.1, cpast v0.7.1 ([`963e502`](https://github.com/rootCircle/cpast_mono/commit/963e502270f0a01c5e985012847abbe0e3d3551b))
    - Release ccode_runner v0.2.0, clex v0.2.1, cpast v0.7.1 ([`3a51aa2`](https://github.com/rootCircle/cpast_mono/commit/3a51aa22d214a8a10dfdee47f3a23f965a0744b2))
    - Cpast_cli clex ccode_runner ([`8a000e0`](https://github.com/rootCircle/cpast_mono/commit/8a000e047deebefdbe34b6c52656c342f149f099))
    - Use atomicbool instead of mutex in compile_and_test ([`27ba21a`](https://github.com/rootCircle/cpast_mono/commit/27ba21a737a84005359317eb48cca12100405b32))
    - Catch os 32 broken pipe errors ([`be396e1`](https://github.com/rootCircle/cpast_mono/commit/be396e164ba64e5cda157ce5ad4d93fe503bf976))
    - Improves error types ([`0d1e7e0`](https://github.com/rootCircle/cpast_mono/commit/0d1e7e089c9f682a95918feddd139a0e33f9d67a))
    - Introduce --debug flag (closes #5) ([`18f1c51`](https://github.com/rootCircle/cpast_mono/commit/18f1c5182c4fd105242aeb7f851edbbeafd778d7))
    - New file store interface and mig to rust 2024 ([`a29a4c1`](https://github.com/rootCircle/cpast_mono/commit/a29a4c1da0732dbf2e9cf3f86873a635b7896592))
    - Allow pipe of generated testcases easily ([`94458dd`](https://github.com/rootCircle/cpast_mono/commit/94458dd0ae5d90e46b6871a7b6eca43ec472d107))
    - Rename to cpast_mono from cpast ([`e98a8df`](https://github.com/rootCircle/cpast_mono/commit/e98a8df53a173d3a51ec2a30cf126802793d0990))
    - Ignore other program file used for testing ([`1a1d511`](https://github.com/rootCircle/cpast_mono/commit/1a1d5113a4797c98ce15f4c8467e3807806139a8))
    - Modified/add README for better segregation ([`e670b0c`](https://github.com/rootCircle/cpast_mono/commit/e670b0ca127f2755ea7ad090f0283cc2bf4cdbc7))
    - Update msrv ([`139c68a`](https://github.com/rootCircle/cpast_mono/commit/139c68a9a1f7178749e6297875fd01437d8b4ac4))
    - Use worspace deps ([`0a04f6f`](https://github.com/rootCircle/cpast_mono/commit/0a04f6f80d8f1c544aeee6fad96a8c366dd2b9ca))
    - Fix failing clipboard test pipeline ([`05cc73b`](https://github.com/rootCircle/cpast_mono/commit/05cc73bdc5887c67fdf86d02df60fc90e2109683))
    - Remove clipboard feature ([`ebec48d`](https://github.com/rootCircle/cpast_mono/commit/ebec48d4366044ecc318cd99afb67dc4f16613ee))
    - Refactor cpast into ccode refactor and cli ([`61afca4`](https://github.com/rootCircle/cpast_mono/commit/61afca4da7d3df0e59fb9ac8b018a476fd1707f2))
    - Fix failing ci ([`c4e4fe1`](https://github.com/rootCircle/cpast_mono/commit/c4e4fe1a126481c850a4f78bd4011125ff988b06))
    - Add cpastcord ([`a898f15`](https://github.com/rootCircle/cpast_mono/commit/a898f15e10691ff4d2389ba9baa1eebaf81c6421))
    - New release ([`bd06417`](https://github.com/rootCircle/cpast_mono/commit/bd06417f6935b916ab6647ddbb40880fd9388c7d))
    - Move to monorepo ([`6d491f5`](https://github.com/rootCircle/cpast_mono/commit/6d491f5355fb74a14cd556d6d777a070bbb1f007))
    - Restructing folder structure cpast ([`0319931`](https://github.com/rootCircle/cpast_mono/commit/03199312b8347f21e93b44dab9cbce3c538182f1))
</details>

## 0.6.0

Released on : Aug 29, 2024

Clex

Introducing custom character types in String, breaking syntax!

Now, for custom character sets use `@CH_ALL@`, `@CH_UPPER@` etc(as found in [Clex Language Specs](https://github.com/rootCircle/cpast_mono/blob/7b999d957af246e03d9e7d258fab1fa4e21cb684/docs/clex/CLEX_LANG_SPECS.md)) or literal string like `'abc'` it will generate either of abc!

We also _dropped Character Type_ as it use was not profound! For using Character Type simply replace it with `S[1,]`

## 0.5.0

Released on : Aug 28, 2024

Critical Bug Fix:

- [Critical High] Due to race condition, `cpast test ...` might leave a
      orphan child process in non `--no-stop` cases! This will eventually
      eat all your system memory and potentially crash it as well! This
      commit fixes that as well!

Major Changes:

- improved error propagation, don't exit before erroring
- improved error types
- (breaking) modify public and private function signature to accommodate
      error propagation
- introduce CPAST_DEBUG env, to reduce verbosity of Success Testcase
      message! It's now disabled by default! To enable use `CPAST_DEBUG=1
      cpast test ...`

## 0.4.1

Released on : Aug 07, 2024

- Shell completion support! Generate using 
  - zsh: `cpast --completions=zsh | sudo tee /usr/local/share/zsh/site-functions/_cpast`
  - fish: `cpast --completions=fish > ~/.local/share/fish/generated_completions/cpast.fish`
  - bash: `cpast --completions=bash | sudo tee /etc/bash_completion.d/cpast.bash`

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

