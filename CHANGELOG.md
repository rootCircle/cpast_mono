# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

For changelogs of packages, see:

- [cpast](./cpast/CHANGELOG.md)
- [clex_gen](./clex_gen/CHANGELOG.md)
- [ccode_runner](./ccode_runner/CHANGELOG.md)

## 0.8.0 [Unreleased] (2025-02-23)

### clex_gen

- Now, string modifier accepts escape characters as well like `\n`, `\t`, `\r`, `\\`, `\'`, `\"`, `\0` `\a`, `\b`, `\f` and `\v` etc!
- Breaking: Modified string modifier syntax to support minimum and maximum length constraints:
  - New format: `S[min,max,charset]` (e.g., `S[10,10,@CH_UPPER@]`)
  - Old format: `S[length,charset]` is now deprecated
  - This change enables more precise string length control in pattern generation

### clex_llm

- Switched to Gemini2_0Flash model from Gemini1_0

## 0.7.1 (2025-02-22)

### cpast_cli

- Introduced `--debug` flag to enable debug mode for cpast_cli. This is breaking changes that replaces `CPAST_DEBUG` environment variable.
