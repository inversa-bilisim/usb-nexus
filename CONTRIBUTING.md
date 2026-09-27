# Contributing

**English** · [Türkçe](CONTRIBUTING.tr.md)

Contributions are welcome!

1. Open an issue describing what you want to do (not needed for small fixes).
2. Work on a separate branch and make sure `cargo fmt`, `cargo clippy --workspace` and
   `cargo test --workspace` pass (see `CLAUDE.md` for the full list of checks).
3. Open a pull request.

## License and source rules

- All contributions are accepted under the project's license, **GPL-3.0-or-later**.
- Start new source files with the SPDX header:
  ```rust
  // SPDX-License-Identifier: GPL-3.0-or-later
  // Copyright (C) 2026 USB Nexus contributors
  ```
- Code comments and documentation inside source files are in English. Every user-visible text
  goes into `locales/*.ftl`, in every language; the tests fail otherwise.
- When taking code from other projects, make sure its license is compatible with GPL-3.0 and
  credit the source, keeping the original copyright lines. **GPL-2.0-only** code (for example
  the Linux kernel) cannot be combined with GPL-3.0; such code may be read for reference only.
- Third-party binaries (for example Windows drivers) are kept under `packaging/` together with
  their license texts and a note on where the source code is available.
