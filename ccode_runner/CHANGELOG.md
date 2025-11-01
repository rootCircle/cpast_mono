# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Time limit support for program execution to prevent infinite loops (all platforms)
- Memory limit support for program execution (all platforms)
  - Unix/Linux/macOS: Native OS enforcement via `setrlimit(RLIMIT_AS)`
  - Windows: Active monitoring via background thread using `sysinfo`
- New `ExecutionLimits` struct with builder pattern for configuring limits
- New error types `TimeLimitExceeded` and `MemoryLimitExceeded`
- Extended API methods: `new_with_limits`, `new_from_text_with_limits`, `new_from_custom_dest_with_limits`
- Comprehensive test suite for execution limits (13 tests with 100% patch coverage)
- Example program demonstrating execution limits usage
- Documentation updates for time and memory limit features

### Dependencies

- Added `wait-timeout` v0.2 for cross-platform timeout support
- Added `libc` v0.2 for Unix-specific memory limiting via `setrlimit`
- Added `sysinfo` v0.31 for cross-platform memory monitoring (Windows support)

## v0.3.6 (2025-09-24)

### Chore

 - <csr-id-d3c932771728a95ebd89991f221f0000a442eaf1/> enhance string normalization and comparison functions with comprehensive tests

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release over the course of 43 calendar days.
 - 43 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Enhance string normalization and comparison functions with comprehensive tests ([`d3c9327`](https://github.com/rootCircle/cpast_mono/commit/d3c932771728a95ebd89991f221f0000a442eaf1))
</details>

## v0.3.5 (2025-08-11)

<csr-id-950ed853366f835e1d552f3c7434523a9977dbba/>
<csr-id-284588063fdf8bb686ad6f79ac7313f2e6b66fb7/>

### Other

 - <csr-id-950ed853366f835e1d552f3c7434523a9977dbba/> cargo sort -w
 - <csr-id-284588063fdf8bb686ad6f79ac7313f2e6b66fb7/> format! lint fix

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 42 calendar days.
 - 148 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release ccode_runner v0.3.5, clex_gen v0.3.4, clex_llm v0.3.2, cscrapper v0.1.3, cpast v0.10.4 ([`291b2fb`](https://github.com/rootCircle/cpast_mono/commit/291b2fbce9bcfaeda1e608f6e8195e3fc4ef999f))
    - Cargo sort -w ([`950ed85`](https://github.com/rootCircle/cpast_mono/commit/950ed853366f835e1d552f3c7434523a9977dbba))
    - Format! lint fix ([`2845880`](https://github.com/rootCircle/cpast_mono/commit/284588063fdf8bb686ad6f79ac7313f2e6b66fb7))
    - Fix faling builds ([`aa689ca`](https://github.com/rootCircle/cpast_mono/commit/aa689ca47e1c97026a81f54503160ee68ebf7829))
</details>

## v0.3.4 (2025-03-15)

### New Features

 - <csr-id-02cb1cef4ccd3f20dff7ce5e45ef22df675e4c05/> make 'utoipa' optional and add 'api' feature to ccode_runner; update LanguageName enum to conditionally derive ToSchema

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release ccode_runner v0.3.4, cscrapper v0.1.0, cpast v0.10.0 ([`51e0827`](https://github.com/rootCircle/cpast_mono/commit/51e08276e821034082e49dc77b9ddd031b84ca0e))
    - Make 'utoipa' optional and add 'api' feature to ccode_runner; update LanguageName enum to conditionally derive ToSchema ([`02cb1ce`](https://github.com/rootCircle/cpast_mono/commit/02cb1cef4ccd3f20dff7ce5e45ef22df675e4c05))
</details>

## v0.3.3 (2025-03-15)

### New Features

 - <csr-id-d1f3bcfce5187879268726170447a032c3e95a40/> expose default codegen language type
   feat(cpast): --problem_url args in ai and test routes
   
   feat(cscrapper): new ProgramStore api new_from_language
   
   chore(cpast_api): updated schema to store language name in cache and use language from clex_llm instead of c++ as default
 - <csr-id-1f866fcf819af5df4c51b9f1aacf9607b6ff47e8/> add sample programs and tests for multiple languages including Java, C, C++, Python, Ruby, JavaScript, and Rust
 - <csr-id-976089bf07b7556823c32621ca9c8da98850932d/> enhance new_from_custom_dest to support Java file naming rules and improve error handling
 - <csr-id-626c3ce60e1cbc3dcd25d92c5298413650d70553/> add new error types for invalid file names and empty temporary directories
 - <csr-id-6a2a02cd18c11bb72b4acdf5ed816a0bc70c333a/> expose from_text from_custom_dest api to public
 - <csr-id-3488984dfc9a81df4538fac6d824a9bda3001209/> code compilation to temp dir

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release over the course of 1 calendar day.
 - 13 days passed between releases.
 - 6 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release ccode_runner v0.3.3, clex_llm v0.3.0, cscrapper v0.1.0, cpast v0.10.0, safety bump cpast v0.10.0 ([`3c24521`](https://github.com/rootCircle/cpast_mono/commit/3c245215b88f1f7e15764bb7e5756929825ec538))
    - Expose default codegen language type ([`d1f3bcf`](https://github.com/rootCircle/cpast_mono/commit/d1f3bcfce5187879268726170447a032c3e95a40))
    - Add sample programs and tests for multiple languages including Java, C, C++, Python, Ruby, JavaScript, and Rust ([`1f866fc`](https://github.com/rootCircle/cpast_mono/commit/1f866fcf819af5df4c51b9f1aacf9607b6ff47e8))
    - Enhance new_from_custom_dest to support Java file naming rules and improve error handling ([`976089b`](https://github.com/rootCircle/cpast_mono/commit/976089bf07b7556823c32621ca9c8da98850932d))
    - Feat(java_classname): add utility to extract public class name from Java source text feat(ccode_runner): enhance source file naming for Java and random languages feat(cpast): add tests for Java public class extraction and code evaluation ([`35c6c11`](https://github.com/rootCircle/cpast_mono/commit/35c6c116e8087c1d7331bbfde3d4dc0bc1da5b90))
    - Add new error types for invalid file names and empty temporary directories ([`626c3ce`](https://github.com/rootCircle/cpast_mono/commit/626c3ce60e1cbc3dcd25d92c5298413650d70553))
    - Expose from_text from_custom_dest api to public ([`6a2a02c`](https://github.com/rootCircle/cpast_mono/commit/6a2a02cd18c11bb72b4acdf5ed816a0bc70c333a))
    - Code compilation to temp dir ([`3488984`](https://github.com/rootCircle/cpast_mono/commit/3488984dfc9a81df4538fac6d824a9bda3001209))
</details>

## v0.3.2 (2025-03-01)

### Documentation

 - <csr-id-ea9fa36e4852bd8af718fe6abe6190be725ed292/> rust docs

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 5 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release ccode_runner v0.3.2, clex_gen v0.3.2, clex_llm v0.2.2, cpast v0.9.2 ([`325d8c1`](https://github.com/rootCircle/cpast_mono/commit/325d8c11588daaad4678e72aac665b58f32f119e))
    - Rust docs ([`ea9fa36`](https://github.com/rootCircle/cpast_mono/commit/ea9fa36e4852bd8af718fe6abe6190be725ed292))
</details>

## v0.3.1 (2025-02-24)

### Reverted

 - <csr-id-5412a2598514f36495a853e177889e753bfbc01f/> revert to blocking api in std::process::Command due to perf degredation

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release ccode_runner v0.3.1, clex_gen v0.3.1, cpast v0.9.1 ([`0ff6d5d`](https://github.com/rootCircle/cpast_mono/commit/0ff6d5d65fbd5c65dbf7edf1e82c5c87818c3308))
    - Revert to blocking api in std::process::Command due to perf degredation ([`5412a25`](https://github.com/rootCircle/cpast_mono/commit/5412a2598514f36495a853e177889e753bfbc01f))
</details>

## v0.3.0 (2025-02-24)

### New Features

 - <csr-id-936d57702cb88bbb028fd66f69d8508e6ef8093f/> moved to tokio::process::Command async framework

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release ccode_runner v0.3.0, cpast v0.9.0 ([`abfdf15`](https://github.com/rootCircle/cpast_mono/commit/abfdf156cfa1bceb09a65e6ef03a22c39e7e26aa))
    - Moved to tokio::process::Command async framework ([`936d577`](https://github.com/rootCircle/cpast_mono/commit/936d57702cb88bbb028fd66f69d8508e6ef8093f))
</details>

## v0.2.2 (2025-02-23)

### Documentation

 - <csr-id-0d6e667a7a3087106e377efd5c2c96881f63caa7/> improved shields badges and README

### New Features

 - <csr-id-7fbe621497981a93f390f1857537b540420c8d18/> introduce ai subcommand to generate clex form input format and constraints
 - <csr-id-d7f610314c38fdad56d297a1371a72e343085212/> introduce ai subcommand to generate clex form input format and constraints

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 1 day passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release ccode_runner v0.2.2, clex_llm v0.2.0, cpast v0.8.0 ([`4b064ed`](https://github.com/rootCircle/cpast_mono/commit/4b064ed89427efe30093dfc0432380945436f8e0))
    - Introduce ai subcommand to generate clex form input format and constraints ([`d7f6103`](https://github.com/rootCircle/cpast_mono/commit/d7f610314c38fdad56d297a1371a72e343085212))
    - Improved shields badges and README ([`0d6e667`](https://github.com/rootCircle/cpast_mono/commit/0d6e667a7a3087106e377efd5c2c96881f63caa7))
</details>

## v0.2.1 (2025-02-23)

### New Features

 - <csr-id-bc8f08ba637c113645a417d558e149dbe16bdd3a/> introduce ai subcommand to generate clex form input format and constraints

### Documentation

 - <csr-id-0d6e667a7a3087106e377efd5c2c96881f63caa7/> improved shields badges and README

## v0.2.0 (2025-02-22)

<csr-id-8a000e047deebefdbe34b6c52656c342f149f099/>
<csr-id-e98a8df53a173d3a51ec2a30cf126802793d0990/>
<csr-id-139c68a9a1f7178749e6297875fd01437d8b4ac4/>
<csr-id-0a04f6f80d8f1c544aeee6fad96a8c366dd2b9ca/>

### Documentation

 - <csr-id-e670b0ca127f2755ea7ad090f0283cc2bf4cdbc7/> modified/add README for better segregation

### New Features

 - <csr-id-0d1e7e089c9f682a95918feddd139a0e33f9d67a/> improves error types
 - <csr-id-4a2df5d564aa4a47229c220f98a927f10db860b2/> improve the error propogation
 - <csr-id-18f1c5182c4fd105242aeb7f851edbbeafd778d7/> introduce --debug flag (closes #5)
 - <csr-id-a29a4c1da0732dbf2e9cf3f86873a635b7896592/> new file store interface and mig to rust 2024
 - <csr-id-0dd1fd71dc0b34a67393d94e9ecdd9726c2c5146/> implemented GET POST /api/v1/share with tests
 - <csr-id-60e5058e0cd777912034718249a067cce31d1398/> remove old routes and introduce share_code route
 - <csr-id-61afca4da7d3df0e59fb9ac8b018a476fd1707f2/> refactor cpast into ccode refactor and cli

### Other

 - <csr-id-8a000e047deebefdbe34b6c52656c342f149f099/> cpast_cli clex ccode_runner
 - <csr-id-e98a8df53a173d3a51ec2a30cf126802793d0990/> rename to cpast_mono from cpast
 - <csr-id-139c68a9a1f7178749e6297875fd01437d8b4ac4/> update msrv
 - <csr-id-0a04f6f80d8f1c544aeee6fad96a8c366dd2b9ca/> use worspace deps

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 18 commits contributed to the release over the course of 116 calendar days.
 - 12 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release ccode_runner v0.2.0, clex v0.2.1, cpast v0.7.1 ([`7e750cc`](https://github.com/rootCircle/cpast_mono/commit/7e750cc72b592bd491f4f503fc72c19043934f1b))
    - Release ccode_runner v0.2.0, clex v0.2.1, cpast v0.7.1 ([`2cfb445`](https://github.com/rootCircle/cpast_mono/commit/2cfb44521a215d57afe95139a830ed442518e2b8))
    - Release ccode_runner v0.2.0, clex v0.2.1, cpast v0.7.1 ([`447fbef`](https://github.com/rootCircle/cpast_mono/commit/447fbef5fb82b81391a2a8a6e827e3870756f961))
    - Release ccode_runner v0.2.0, clex v0.2.1, cpast v0.7.1 ([`a5478ea`](https://github.com/rootCircle/cpast_mono/commit/a5478ea8c1548147655142d73b6d82e8d7676cb7))
    - Release ccode_runner v0.2.0, clex v0.2.1, cpast v0.7.1 ([`963e502`](https://github.com/rootCircle/cpast_mono/commit/963e502270f0a01c5e985012847abbe0e3d3551b))
    - Release ccode_runner v0.2.0, clex v0.2.1, cpast v0.7.1 ([`3a51aa2`](https://github.com/rootCircle/cpast_mono/commit/3a51aa22d214a8a10dfdee47f3a23f965a0744b2))
    - Cpast_cli clex ccode_runner ([`8a000e0`](https://github.com/rootCircle/cpast_mono/commit/8a000e047deebefdbe34b6c52656c342f149f099))
    - Improves error types ([`0d1e7e0`](https://github.com/rootCircle/cpast_mono/commit/0d1e7e089c9f682a95918feddd139a0e33f9d67a))
    - Improve the error propogation ([`4a2df5d`](https://github.com/rootCircle/cpast_mono/commit/4a2df5d564aa4a47229c220f98a927f10db860b2))
    - Introduce --debug flag (closes #5) ([`18f1c51`](https://github.com/rootCircle/cpast_mono/commit/18f1c5182c4fd105242aeb7f851edbbeafd778d7))
    - New file store interface and mig to rust 2024 ([`a29a4c1`](https://github.com/rootCircle/cpast_mono/commit/a29a4c1da0732dbf2e9cf3f86873a635b7896592))
    - Rename to cpast_mono from cpast ([`e98a8df`](https://github.com/rootCircle/cpast_mono/commit/e98a8df53a173d3a51ec2a30cf126802793d0990))
    - Implemented GET POST /api/v1/share with tests ([`0dd1fd7`](https://github.com/rootCircle/cpast_mono/commit/0dd1fd71dc0b34a67393d94e9ecdd9726c2c5146))
    - Remove old routes and introduce share_code route ([`60e5058`](https://github.com/rootCircle/cpast_mono/commit/60e5058e0cd777912034718249a067cce31d1398))
    - Modified/add README for better segregation ([`e670b0c`](https://github.com/rootCircle/cpast_mono/commit/e670b0ca127f2755ea7ad090f0283cc2bf4cdbc7))
    - Update msrv ([`139c68a`](https://github.com/rootCircle/cpast_mono/commit/139c68a9a1f7178749e6297875fd01437d8b4ac4))
    - Use worspace deps ([`0a04f6f`](https://github.com/rootCircle/cpast_mono/commit/0a04f6f80d8f1c544aeee6fad96a8c366dd2b9ca))
    - Refactor cpast into ccode refactor and cli ([`61afca4`](https://github.com/rootCircle/cpast_mono/commit/61afca4da7d3df0e59fb9ac8b018a476fd1707f2))
</details>

