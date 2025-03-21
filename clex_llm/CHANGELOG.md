# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.3.1 (2025-03-17)

### Bug Fixes

 - <csr-id-cc1fddcf1ccf015837f91721e19d28a303c16597/> correct language name assignment in test_code_generator

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
    - Correct language name assignment in test_code_generator ([`cc1fddc`](https://github.com/rootCircle/cpast_mono/commit/cc1fddcf1ccf015837f91721e19d28a303c16597))
</details>

## v0.3.0 (2025-03-15)

### New Features

 - <csr-id-d1f3bcfce5187879268726170447a032c3e95a40/> expose default codegen language type
   feat(cpast): --problem_url args in ai and test routes
   
   feat(cscrapper): new ProgramStore api new_from_language
   
   chore(cpast_api): updated schema to store language name in cache and use language from clex_llm instead of c++ as default
 - <csr-id-a7be429d63454f278237173dab38f8dd1b522af2/> update code generation to support C++ solutions and enhance input handling
 - <csr-id-ab31151dec6626ea67cc24ddab8ff390b9989282/> add caching for code evaluation and new evaluate route with_platform
 - <csr-id-06c5c95dd7941ca0eb2dd7ac96b503feba4a0a53/> introduce new code and clex generators with updated API integration
 - <csr-id-7ea37a311ecc70db654e9b4facc76cfca4a6290a/> add LLM API key configuration and integrate into evaluation routes

### New Features (BREAKING)

 - <csr-id-e2cdab56fcb473fe24e5e8acbea83c1703e97cb7/> update API key references from GEMINI_API_KEY to GOOGLE_API_KEY and add secrets setup script for CI

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 1 calendar day.
 - 13 days passed between releases.
 - 6 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release ccode_runner v0.3.3, clex_llm v0.3.0, cscrapper v0.1.0, cpast v0.10.0, safety bump cpast v0.10.0 ([`3c24521`](https://github.com/rootCircle/cpast_mono/commit/3c245215b88f1f7e15764bb7e5756929825ec538))
    - Expose default codegen language type ([`d1f3bcf`](https://github.com/rootCircle/cpast_mono/commit/d1f3bcfce5187879268726170447a032c3e95a40))
    - Update code generation to support C++ solutions and enhance input handling ([`a7be429`](https://github.com/rootCircle/cpast_mono/commit/a7be429d63454f278237173dab38f8dd1b522af2))
    - Add caching for code evaluation and new evaluate route with_platform ([`ab31151`](https://github.com/rootCircle/cpast_mono/commit/ab31151dec6626ea67cc24ddab8ff390b9989282))
    - Introduce new code and clex generators with updated API integration ([`06c5c95`](https://github.com/rootCircle/cpast_mono/commit/06c5c95dd7941ca0eb2dd7ac96b503feba4a0a53))
    - Update API key references from GEMINI_API_KEY to GOOGLE_API_KEY and add secrets setup script for CI ([`e2cdab5`](https://github.com/rootCircle/cpast_mono/commit/e2cdab56fcb473fe24e5e8acbea83c1703e97cb7))
    - Add LLM API key configuration and integrate into evaluation routes ([`7ea37a3`](https://github.com/rootCircle/cpast_mono/commit/7ea37a311ecc70db654e9b4facc76cfca4a6290a))
</details>

## v0.2.2 (2025-03-01)

<csr-id-f93b38cd00306356503de16b84202333ea3baee4/>

### Chore

 - <csr-id-f93b38cd00306356503de16b84202333ea3baee4/> update default range values to i32_min, i32_max and u32_min, u32_max; improve completions support and documentation

### Documentation

 - <csr-id-ea9fa36e4852bd8af718fe6abe6190be725ed292/> rust docs

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 6 days passed between releases.
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

## v0.2.1 (2025-02-23)

### Documentation

 - <csr-id-5d48fdc64ef4d36aacfa3b24aa893bfc02ce6eb5/> add badges for clex_llm

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release clex_llm v0.2.1, cpast v0.8.1 ([`8bfc4c8`](https://github.com/rootCircle/cpast_mono/commit/8bfc4c8d37ea0ba4a495ef8f8eab7b955354c259))
    - Release clex_llm v0.2.0, cpast v0.8.0 ([`f9beff1`](https://github.com/rootCircle/cpast_mono/commit/f9beff1c3efe2ba5ec8347b7f0dbf71ddf66ef1b))
    - Add badges for clex_llm ([`5d48fdc`](https://github.com/rootCircle/cpast_mono/commit/5d48fdc64ef4d36aacfa3b24aa893bfc02ce6eb5))
</details>

## v0.2.0 (2025-02-23)

<csr-id-e98a8df53a173d3a51ec2a30cf126802793d0990/>
<csr-id-139c68a9a1f7178749e6297875fd01437d8b4ac4/>
<csr-id-0a04f6f80d8f1c544aeee6fad96a8c366dd2b9ca/>
<csr-id-9a63c718ab3848503f75ff7e9bb1b5fbc022021b/>

### Documentation

 - <csr-id-e670b0ca127f2755ea7ad090f0283cc2bf4cdbc7/> modified/add README for better segregation
 - <csr-id-5d48fdc64ef4d36aacfa3b24aa893bfc02ce6eb5/> add badges for clex_llm

### New Features

 - <csr-id-ebc1e6e3d20e2fdee80ba6c0cb780f2c0d4db06e/> introduce ai subcommand to generate clex form input format and constraints
 - <csr-id-1fa604a848dc960908c7148efde4621a38f2a573/> add support for min,max length in string as well espace characters in custom charsets and updated llm model to gemini 2_0flash
 - <csr-id-18f1c5182c4fd105242aeb7f851edbbeafd778d7/> introduce --debug flag (closes #5)
 - <csr-id-a29a4c1da0732dbf2e9cf3f86873a635b7896592/> new file store interface and mig to rust 2024
 - <csr-id-d473aa8e95145a2815e910c877f98788c2cb5bc5/> integrated clex llm with gemini
 - <csr-id-bc8f08ba637c113645a417d558e149dbe16bdd3a/> introduce ai subcommand to generate clex form input format and constraints
 - <csr-id-7fbe621497981a93f390f1857537b540420c8d18/> introduce ai subcommand to generate clex form input format and constraints
 - <csr-id-d7f610314c38fdad56d297a1371a72e343085212/> introduce ai subcommand to generate clex form input format and constraints

### Other

 - <csr-id-e98a8df53a173d3a51ec2a30cf126802793d0990/> rename to cpast_mono from cpast
 - <csr-id-139c68a9a1f7178749e6297875fd01437d8b4ac4/> update msrv
 - <csr-id-0a04f6f80d8f1c544aeee6fad96a8c366dd2b9ca/> use worspace deps

### Refactor

 - <csr-id-9a63c718ab3848503f75ff7e9bb1b5fbc022021b/> rename clex package to clex_gen

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 11 commits contributed to the release over the course of 117 calendar days.
 - 10 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release ccode_runner v0.2.2, clex_llm v0.2.0, cpast v0.8.0 ([`4b064ed`](https://github.com/rootCircle/cpast_mono/commit/4b064ed89427efe30093dfc0432380945436f8e0))
    - Introduce ai subcommand to generate clex form input format and constraints ([`d7f6103`](https://github.com/rootCircle/cpast_mono/commit/d7f610314c38fdad56d297a1371a72e343085212))
    - Add support for min,max length in string as well espace characters in custom charsets and updated llm model to gemini 2_0flash ([`1fa604a`](https://github.com/rootCircle/cpast_mono/commit/1fa604a848dc960908c7148efde4621a38f2a573))
    - Rename clex package to clex_gen ([`9a63c71`](https://github.com/rootCircle/cpast_mono/commit/9a63c718ab3848503f75ff7e9bb1b5fbc022021b))
    - Introduce --debug flag (closes #5) ([`18f1c51`](https://github.com/rootCircle/cpast_mono/commit/18f1c5182c4fd105242aeb7f851edbbeafd778d7))
    - New file store interface and mig to rust 2024 ([`a29a4c1`](https://github.com/rootCircle/cpast_mono/commit/a29a4c1da0732dbf2e9cf3f86873a635b7896592))
    - Rename to cpast_mono from cpast ([`e98a8df`](https://github.com/rootCircle/cpast_mono/commit/e98a8df53a173d3a51ec2a30cf126802793d0990))
    - Modified/add README for better segregation ([`e670b0c`](https://github.com/rootCircle/cpast_mono/commit/e670b0ca127f2755ea7ad090f0283cc2bf4cdbc7))
    - Update msrv ([`139c68a`](https://github.com/rootCircle/cpast_mono/commit/139c68a9a1f7178749e6297875fd01437d8b4ac4))
    - Use worspace deps ([`0a04f6f`](https://github.com/rootCircle/cpast_mono/commit/0a04f6f80d8f1c544aeee6fad96a8c366dd2b9ca))
    - Integrated clex llm with gemini ([`d473aa8`](https://github.com/rootCircle/cpast_mono/commit/d473aa8e95145a2815e910c877f98788c2cb5bc5))
</details>

