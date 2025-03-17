# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.1.2 (2025-03-17)

### Chore

 - <csr-id-4f809172dcf7cf104859a5a3567547e99900930d/> update non-cloudflare blocked codeforces mirror(slow)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release over the course of 1 calendar day.
 - 1 day passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Update non-cloudflare blocked codeforces mirror(slow) ([`4f80917`](https://github.com/rootCircle/cpast_mono/commit/4f809172dcf7cf104859a5a3567547e99900930d))
</details>

## v0.1.1 (2025-03-15)

### Bug Fixes

 - <csr-id-07b16ae8573d8376d4b2b3053d594699602ab8e5/> increase timeout

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release cscrapper v0.1.1 ([`f90f9a3`](https://github.com/rootCircle/cpast_mono/commit/f90f9a3506aad49e6a0f1d0cb0f666b117a1cfd8))
    - Increase timeout ([`07b16ae`](https://github.com/rootCircle/cpast_mono/commit/07b16ae8573d8376d4b2b3053d594699602ab8e5))
</details>

## v0.1.0 (2025-03-15)

<csr-id-6717bb04b76eca27606ccfc893eee781853a45e7/>
<csr-id-139c68a9a1f7178749e6297875fd01437d8b4ac4/>
<csr-id-0a04f6f80d8f1c544aeee6fad96a8c366dd2b9ca/>

### Chore

 - <csr-id-6717bb04b76eca27606ccfc893eee781853a45e7/> remove thread sleep in codechef (fixes #10)

### Bug Fixes

 - <csr-id-e211a8d39e517cb46597f2c2b2f3e72ccfef8ff5/> update keywords in Cargo.toml for accuracy

### Documentation

 - <csr-id-1dede188964146586f0eebfff4df5793b9a5d846/> fix toml warning in readme
 - <csr-id-e670b0ca127f2755ea7ad090f0283cc2bf4cdbc7/> modified/add README for better segregation

### New Features

 - <csr-id-06c5c95dd7941ca0eb2dd7ac96b503feba4a0a53/> introduce new code and clex generators with updated API integration
 - <csr-id-9cf14537ace1758e3dc55cc5cb3a40db9d29027e/> add problem URL parsing for CodeChef and Codeforces with tests
 - <csr-id-18f1c5182c4fd105242aeb7f851edbbeafd778d7/> introduce --debug flag (closes #5)
 - <csr-id-a29a4c1da0732dbf2e9cf3f86873a635b7896592/> new file store interface and mig to rust 2024
 - <csr-id-d1f5932c0b045fb5d01acf32c9310c44216ad58f/> scrapper port to rust
 - <csr-id-02cb1cef4ccd3f20dff7ce5e45ef22df675e4c05/> make 'utoipa' optional and add 'api' feature to ccode_runner; update LanguageName enum to conditionally derive ToSchema
 - <csr-id-0d908d6195630af425df9b083de6dcaebf101bfa/> add description, categories, and keywords to Cargo.toml

### Other

 - <csr-id-139c68a9a1f7178749e6297875fd01437d8b4ac4/> update msrv
 - <csr-id-0a04f6f80d8f1c544aeee6fad96a8c366dd2b9ca/> use worspace deps

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 17 commits contributed to the release over the course of 137 calendar days.
 - 13 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release cscrapper v0.1.0, cpast v0.10.0 ([`b7f3958`](https://github.com/rootCircle/cpast_mono/commit/b7f3958d3e6b43f80a40aaea5da3e5e718a6d3b0))
    - Update keywords in Cargo.toml for accuracy ([`e211a8d`](https://github.com/rootCircle/cpast_mono/commit/e211a8d39e517cb46597f2c2b2f3e72ccfef8ff5))
    - Release cscrapper v0.1.0, cpast v0.10.0 ([`b88b38e`](https://github.com/rootCircle/cpast_mono/commit/b88b38e61da71e3318cc8e72fe8812b70977802c))
    - Add description, categories, and keywords to Cargo.toml ([`0d908d6`](https://github.com/rootCircle/cpast_mono/commit/0d908d6195630af425df9b083de6dcaebf101bfa))
    - Release ccode_runner v0.3.4, cscrapper v0.1.0, cpast v0.10.0 ([`51e0827`](https://github.com/rootCircle/cpast_mono/commit/51e08276e821034082e49dc77b9ddd031b84ca0e))
    - Make 'utoipa' optional and add 'api' feature to ccode_runner; update LanguageName enum to conditionally derive ToSchema ([`02cb1ce`](https://github.com/rootCircle/cpast_mono/commit/02cb1cef4ccd3f20dff7ce5e45ef22df675e4c05))
    - Introduce new code and clex generators with updated API integration ([`06c5c95`](https://github.com/rootCircle/cpast_mono/commit/06c5c95dd7941ca0eb2dd7ac96b503feba4a0a53))
    - Remove thread sleep in codechef (fixes #10) ([`6717bb0`](https://github.com/rootCircle/cpast_mono/commit/6717bb04b76eca27606ccfc893eee781853a45e7))
    - Feat(cscrapper): refactor problem scraper to support async operations and add new migration for clex column (fixes #9) feat(cpast_api): evaluate route with_code_and_platform and tests ([`d460a2c`](https://github.com/rootCircle/cpast_mono/commit/d460a2c07607dec1803f1da9ae55cb5bbfa8a547))
    - Add problem URL parsing for CodeChef and Codeforces with tests ([`9cf1453`](https://github.com/rootCircle/cpast_mono/commit/9cf14537ace1758e3dc55cc5cb3a40db9d29027e))
    - Introduce --debug flag (closes #5) ([`18f1c51`](https://github.com/rootCircle/cpast_mono/commit/18f1c5182c4fd105242aeb7f851edbbeafd778d7))
    - New file store interface and mig to rust 2024 ([`a29a4c1`](https://github.com/rootCircle/cpast_mono/commit/a29a4c1da0732dbf2e9cf3f86873a635b7896592))
    - Fix toml warning in readme ([`1dede18`](https://github.com/rootCircle/cpast_mono/commit/1dede188964146586f0eebfff4df5793b9a5d846))
    - Modified/add README for better segregation ([`e670b0c`](https://github.com/rootCircle/cpast_mono/commit/e670b0ca127f2755ea7ad090f0283cc2bf4cdbc7))
    - Update msrv ([`139c68a`](https://github.com/rootCircle/cpast_mono/commit/139c68a9a1f7178749e6297875fd01437d8b4ac4))
    - Use worspace deps ([`0a04f6f`](https://github.com/rootCircle/cpast_mono/commit/0a04f6f80d8f1c544aeee6fad96a8c366dd2b9ca))
    - Scrapper port to rust ([`d1f5932`](https://github.com/rootCircle/cpast_mono/commit/d1f5932c0b045fb5d01acf32c9310c44216ad58f))
</details>

