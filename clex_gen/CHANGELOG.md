# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.3.2 (2025-03-01)

### Chore

 - <csr-id-f93b38cd00306356503de16b84202333ea3baee4/> update default range values to i32_min, i32_max and u32_min, u32_max; improve completions support and documentation

### Documentation

 - <csr-id-ea9fa36e4852bd8af718fe6abe6190be725ed292/> rust docs

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 5 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
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

### Chore

 - <csr-id-bd06417f6935b916ab6647ddbb40880fd9388c7d/> new release
 - <csr-id-6f1b9982d4b3fd8ec01bf4273a605916dd177bb5/> monorepo setup

### Refactor

 - <csr-id-9a63c718ab3848503f75ff7e9bb1b5fbc022021b/> rename clex package to clex_gen

### Documentation

 - <csr-id-1dede188964146586f0eebfff4df5793b9a5d846/> fix toml warning in readme
 - <csr-id-e670b0ca127f2755ea7ad090f0283cc2bf4cdbc7/> modified/add README for better segregation

### New Features

 - <csr-id-a29a4c1da0732dbf2e9cf3f86873a635b7896592/> new file store interface and mig to rust 2024
 - <csr-id-6d491f5355fb74a14cd556d6d777a070bbb1f007/> move to monorepo

### Other

 - <csr-id-8a000e047deebefdbe34b6c52656c342f149f099/> cpast_cli clex ccode_runner
 - <csr-id-139c68a9a1f7178749e6297875fd01437d8b4ac4/> update msrv
 - <csr-id-0a04f6f80d8f1c544aeee6fad96a8c366dd2b9ca/> use worspace deps

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

