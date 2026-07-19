# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.4.0 (2026-07-20)

### Added

- **Literal Text Injection (`"..."`)**: Support for double-quoted literal text in expressions (e.g. `"YES\n"`, `"# "`) with full escape character support (`\n`, `\t`, `\r`, `\"`, `\\`, etc.).
- **Quantifier Range Repetition (`{min, max}`)**: Support for variable repetition bounds `{min, max}` on primitives and non-capturing groups.
- **Non-Capturing Group Alternation (`(?: ... | ... )`)**: Support for branch alternation inside non-capturing groups using pipe `|`.
- **Flexible String Modifier Forms**: Support for `S[len]`, `S[min, max]`, `S[len, charset]`, and `S[min, max, charset]` syntax.
- **Floating-Point Decimal Bounds (`F[min, max]`)**: Support for literal floating-point bounds (e.g. `F[1.5, 2.7]`) and float back-references.
- **Graph & Permutation Topology Directives**: Native graph generation via `@TREE(N)@`, `@GRAPH(N, M)@`, `@DAG(N, M)@`, and `@PERM(N)@`.
- **Loop Iteration Index Variable (`%i`)**: Support for current loop iteration counter `%i` (with offsets e.g. `%i-1`, `%i+1`) inside repeated non-capturing groups.
- **AST Span Propagation**: AST nodes (`UnitExpression`) now preserve precise `Span` locations for accurate error reporting during parsing and generation.

### Fixed
- **Lexer Memory Leak**: Changed `UnknownCharacter` error variant to use an owned `String` instead of leaking memory via `Box::leak`.
- **Group Parentheses Matching**: Fixed parenthetical nesting checks in `peek_from_current` using a depth counter instead of stack double-pop logic.
- **Group Reference Validation**: Added parse-time validation ensuring back-references (`\N`) refer only to existing, previously-defined capturing groups.

### Test

 - <csr-id-5c6669c7502d48230411d8f7d07f47cacb152e26/> add comprehensive edge cases and Unicode support

### Bug Fixes

 - <csr-id-a0ddf7df3e5da1c717d80f87689514d6a9ecb752/> validate group reference numbers at parse time
 - <csr-id-58a206075c2e01f8e0af8559dce2262a62b5e88b/> fix group parentheses matching in peek_from_current
 - <csr-id-78b4294edb82d51f6eff7ac36578ca3295d58147/> fix memory leak in UnknownCharacter error handling

### New Features

 - <csr-id-5c8f3f84e88059e8757b4dfbfe6ce2757296a71b/> add graph and permutation topology directives
 - <csr-id-5dcc43932291e82f50b3ad44ba0b54f9d50f4a18/> add loop iteration index (%i) support
 - <csr-id-44b82db26a8716c2916ffa9aa3f057087264a000/> add floating-point decimal range bounds
 - <csr-id-e224026b8020001251735240042bfe0c201034a8/> add flexible string modifier syntax
 - <csr-id-d18899d4b03dc6e7048a994c18428c43340a4e02/> add non-capturing group alternation support
 - <csr-id-c4a2563367d63b4c516e7533dfb370f56236e2ae/> add quantifier range repetition support
 - <csr-id-7b98f61c678580bb9e7297a3e5caf8bbcca59c02/> add literal text injection support
 - <csr-id-07267653bdd4c68f983de455eb42dbc40e1dbc8a/> propagate spans to AST nodes for precise error location

### Documentation

 - <csr-id-2b451b7d5ff5d307f568afb8b681ba23be9c505b/> update grammar specifications, README, and LLM prompts for graph topology
 - <csr-id-e0c58529eb03c810c8ccfa032007400c0ed02771/> add changelog entry for grammar optimizations and bugfixes
 - <csr-id-e59787a6665dfbc9be84095715ca78f365468af6/> update FAQs with branch alternation information
 - <csr-id-b9b43fa1d323f449f6d19f9dc25ca2a890f3a9b7/> update grammar specification and README examples

### Chore

 - <csr-id-f2e058f4767aa461b6c9789fd26d187ac65fa074/> pre-release version bump
 - <csr-id-cd93d529006a4ce65b071bdd0c0c8cd362769cb4/> cargo clippy and fmt

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 18 commits contributed to the release.
 - 38 days passed between releases.
 - 18 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Pre-release version bump ([`f2e058f`](https://github.com/rootCircle/cpast_mono/commit/f2e058f4767aa461b6c9789fd26d187ac65fa074))
    - Update grammar specifications, README, and LLM prompts for graph topology ([`2b451b7`](https://github.com/rootCircle/cpast_mono/commit/2b451b7d5ff5d307f568afb8b681ba23be9c505b))
    - Add graph and permutation topology directives ([`5c8f3f8`](https://github.com/rootCircle/cpast_mono/commit/5c8f3f84e88059e8757b4dfbfe6ce2757296a71b))
    - Add loop iteration index (%i) support ([`5dcc439`](https://github.com/rootCircle/cpast_mono/commit/5dcc43932291e82f50b3ad44ba0b54f9d50f4a18))
    - Cargo clippy and fmt ([`cd93d52`](https://github.com/rootCircle/cpast_mono/commit/cd93d529006a4ce65b071bdd0c0c8cd362769cb4))
    - Add comprehensive edge cases and Unicode support ([`5c6669c`](https://github.com/rootCircle/cpast_mono/commit/5c6669c7502d48230411d8f7d07f47cacb152e26))
    - Add changelog entry for grammar optimizations and bugfixes ([`e0c5852`](https://github.com/rootCircle/cpast_mono/commit/e0c58529eb03c810c8ccfa032007400c0ed02771))
    - Update FAQs with branch alternation information ([`e59787a`](https://github.com/rootCircle/cpast_mono/commit/e59787a6665dfbc9be84095715ca78f365468af6))
    - Update grammar specification and README examples ([`b9b43fa`](https://github.com/rootCircle/cpast_mono/commit/b9b43fa1d323f449f6d19f9dc25ca2a890f3a9b7))
    - Add floating-point decimal range bounds ([`44b82db`](https://github.com/rootCircle/cpast_mono/commit/44b82db26a8716c2916ffa9aa3f057087264a000))
    - Add flexible string modifier syntax ([`e224026`](https://github.com/rootCircle/cpast_mono/commit/e224026b8020001251735240042bfe0c201034a8))
    - Add non-capturing group alternation support ([`d18899d`](https://github.com/rootCircle/cpast_mono/commit/d18899d4b03dc6e7048a994c18428c43340a4e02))
    - Add quantifier range repetition support ([`c4a2563`](https://github.com/rootCircle/cpast_mono/commit/c4a2563367d63b4c516e7533dfb370f56236e2ae))
    - Add literal text injection support ([`7b98f61`](https://github.com/rootCircle/cpast_mono/commit/7b98f61c678580bb9e7297a3e5caf8bbcca59c02))
    - Propagate spans to AST nodes for precise error location ([`0726765`](https://github.com/rootCircle/cpast_mono/commit/07267653bdd4c68f983de455eb42dbc40e1dbc8a))
    - Validate group reference numbers at parse time ([`a0ddf7d`](https://github.com/rootCircle/cpast_mono/commit/a0ddf7df3e5da1c717d80f87689514d6a9ecb752))
    - Fix group parentheses matching in peek_from_current ([`58a2060`](https://github.com/rootCircle/cpast_mono/commit/58a206075c2e01f8e0af8559dce2262a62b5e88b))
    - Fix memory leak in UnknownCharacter error handling ([`78b4294`](https://github.com/rootCircle/cpast_mono/commit/78b4294edb82d51f6eff7ac36578ca3295d58147))
</details>

## v0.3.6 (2026-06-12)

<csr-id-17a121b4f9681c379a292f8bff070a661b0b2a75/>

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 17 calendar days.
 - 222 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release ccode_runner v0.3.8, clex_gen v0.3.6, clex_llm v0.3.4, cpast v0.10.6 ([`0b96d8e`](https://github.com/rootCircle/cpast_mono/commit/0b96d8e3a60490285c93db799224ad98129ef353))
    - Release ccode_runner v0.3.8, clex_gen v0.3.6, clex_llm v0.3.4, cpast v0.10.6 ([`38abc24`](https://github.com/rootCircle/cpast_mono/commit/38abc243373d2d5c9b03f5c1c20601fb03790137))
    - Upgrade ([`17a121b`](https://github.com/rootCircle/cpast_mono/commit/17a121b4f9681c379a292f8bff070a661b0b2a75))
</details>

## v0.3.5 (2025-11-02)

### New Features

- Error spans and much better error message.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release.
 - 83 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release clex_gen v0.3.5 ([`5adf07c`](https://github.com/rootCircle/cpast_mono/commit/5adf07c9d783f7460a2572911ba06bc6afe4e2fe))
    - Adjusting changelogs prior to release of clex_gen v0.3.5 ([`b6a494b`](https://github.com/rootCircle/cpast_mono/commit/b6a494b95d5acaa18d0419304ae41ea6d6031472))
    - Merge pull request #50 from rootCircle/copilot/improve-error-messages-clex ([`ce36a37`](https://github.com/rootCircle/cpast_mono/commit/ce36a37df641dcac8d6d3dd0ce468785e5a81a85))
    - Make error messages more friendly with helpful hints and examples ([`6669f09`](https://github.com/rootCircle/cpast_mono/commit/6669f097ff2ee05325e52d6fdab601bebb7e60f3))
    - Fix build error by adding ClexErrorType to ClexError conversion ([`c05e0f7`](https://github.com/rootCircle/cpast_mono/commit/c05e0f71d6d3afcbe4acf03b07f8e49ffef36fc0))
    - Make error messages more ergonomic with cargo/clippy style formatting ([`788854b`](https://github.com/rootCircle/cpast_mono/commit/788854b0b99adeb4e643a127fe4b8fa3d998bb6d))
    - Fix formatting and clippy warnings in error_messages_with_span test ([`228ee97`](https://github.com/rootCircle/cpast_mono/commit/228ee9727c18e864ac1e79244fe9196fb75095a3))
    - Add comprehensive tests for error messages with span information ([`5cf3826`](https://github.com/rootCircle/cpast_mono/commit/5cf3826d20651bcd55ecc35a6c81196273e5417d))
    - Add span/position information to clex error messages ([`d9a9fe4`](https://github.com/rootCircle/cpast_mono/commit/d9a9fe4399212f1d4a1e6c1b215792207f3a43b7))
</details>

## v0.3.4 (2025-08-11)

<csr-id-284588063fdf8bb686ad6f79ac7313f2e6b66fb7/>

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 42 calendar days.
 - 147 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release ccode_runner v0.3.5, clex_gen v0.3.4, clex_llm v0.3.2, cscrapper v0.1.3, cpast v0.10.4 ([`291b2fb`](https://github.com/rootCircle/cpast_mono/commit/291b2fbce9bcfaeda1e608f6e8195e3fc4ef999f))
    - Format! lint fix ([`2845880`](https://github.com/rootCircle/cpast_mono/commit/284588063fdf8bb686ad6f79ac7313f2e6b66fb7))
</details>

## v0.3.3 (2025-03-17)

<csr-id-0abddf0155936c81065388500163b92a209cd522/>

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 16 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release clex_gen v0.3.3, clex_llm v0.3.1, cscrapper v0.1.2, cpast v0.10.3 ([`0d90d92`](https://github.com/rootCircle/cpast_mono/commit/0d90d92652a9c58b73960b5dbc282f332f66a2b4))
    - Update rand to 0.9 ([`0abddf0`](https://github.com/rootCircle/cpast_mono/commit/0abddf0155936c81065388500163b92a209cd522))
</details>

## v0.3.2 (2025-03-01)

<csr-id-f93b38cd00306356503de16b84202333ea3baee4/>

### Documentation

 - <csr-id-ea9fa36e4852bd8af718fe6abe6190be725ed292/> rust docs

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 5 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release ccode_runner v0.3.2, clex_gen v0.3.2, clex_llm v0.2.2, cpast v0.9.2 ([`325d8c1`](https://github.com/rootCircle/cpast_mono/commit/325d8c11588daaad4678e72aac665b58f32f119e))
    - Update default range values to i32_min, i32_max and u32_min, u32_max; improve completions support and documentation ([`f93b38c`](https://github.com/rootCircle/cpast_mono/commit/f93b38cd00306356503de16b84202333ea3baee4))
    - Rust docs ([`ea9fa36`](https://github.com/rootCircle/cpast_mono/commit/ea9fa36e4852bd8af718fe6abe6190be725ed292))
</details>

## v0.3.1 (2025-02-24)

### Documentation

 - <csr-id-0d6e667a7a3087106e377efd5c2c96881f63caa7/> improved shields badges and README

### New Features

 - <csr-id-d7f610314c38fdad56d297a1371a72e343085212/> introduce ai subcommand to generate clex form input format and constraints
 - <csr-id-1fa604a848dc960908c7148efde4621a38f2a573/> add support for min,max length in string as well espace characters in custom charsets and updated llm model to gemini 2_0flash

### Performance

 - <csr-id-09e5ef1dad1dd6fed69463208870025298b1071c/> Reduce mutations by segregating states making `generate_testcase` method functional, and removing the need of cloning in case of `cpast_cli`.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 2 calendar days.
 - 2 days passed between releases.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release ccode_runner v0.3.1, clex_gen v0.3.1, cpast v0.9.1 ([`0ff6d5d`](https://github.com/rootCircle/cpast_mono/commit/0ff6d5d65fbd5c65dbf7edf1e82c5c87818c3308))
    - Reduce mutations by segregating states making `generate_testcase` method functional, and removing the need of cloning in case of `cpast_cli`. ([`09e5ef1`](https://github.com/rootCircle/cpast_mono/commit/09e5ef1dad1dd6fed69463208870025298b1071c))
    - Introduce ai subcommand to generate clex form input format and constraints ([`d7f6103`](https://github.com/rootCircle/cpast_mono/commit/d7f610314c38fdad56d297a1371a72e343085212))
    - Add support for min,max length in string as well espace characters in custom charsets and updated llm model to gemini 2_0flash ([`1fa604a`](https://github.com/rootCircle/cpast_mono/commit/1fa604a848dc960908c7148efde4621a38f2a573))
    - Improved shields badges and README ([`0d6e667`](https://github.com/rootCircle/cpast_mono/commit/0d6e667a7a3087106e377efd5c2c96881f63caa7))
</details>

## v0.3.0 (2025-02-23)

### Documentation

 - <csr-id-0d6e667a7a3087106e377efd5c2c96881f63caa7/> improved shields badges and README

### New Features

 - <csr-id-ebc1e6e3d20e2fdee80ba6c0cb780f2c0d4db06e/> introduce ai subcommand to generate clex form input format and constraints
 - <csr-id-1fa604a848dc960908c7148efde4621a38f2a573/> add support for min,max length in string as well espace characters in custom charsets and updated llm model to gemini 2_0flash
 - <csr-id-bc8f08ba637c113645a417d558e149dbe16bdd3a/> introduce ai subcommand to generate clex form input format and constraints

## v0.2.1 (2025-02-22)

<csr-id-bd06417f6935b916ab6647ddbb40880fd9388c7d/>
<csr-id-6f1b9982d4b3fd8ec01bf4273a605916dd177bb5/>
<csr-id-8a000e047deebefdbe34b6c52656c342f149f099/>
<csr-id-139c68a9a1f7178749e6297875fd01437d8b4ac4/>
<csr-id-0a04f6f80d8f1c544aeee6fad96a8c366dd2b9ca/>
<csr-id-9a63c718ab3848503f75ff7e9bb1b5fbc022021b/>

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release clex_gen v0.2.1, cpast v0.7.1 ([`e50930b`](https://github.com/rootCircle/cpast_mono/commit/e50930bcf32be4bf4a29f6cfea8fb51d72226482))
    - Release clex_gen v0.2.1, cpast v0.7.1 ([`322a597`](https://github.com/rootCircle/cpast_mono/commit/322a5976a09ebb1c49241d08d2b11c07d0d9cd5e))
    - Rename clex package to clex_gen ([`9a63c71`](https://github.com/rootCircle/cpast_mono/commit/9a63c718ab3848503f75ff7e9bb1b5fbc022021b))
</details>

