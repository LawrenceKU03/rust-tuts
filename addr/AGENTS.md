# AGENTS.md

- **Name:** addr
- **Language / Edition:** Rust (Edition 2024)
- **Dependencies:** None

## Commands

- **Build:** `cargo build`
- **Check:** `cargo check`
- **Test:** `cargo test`
- **Lint:** `cargo clippy`

## Architecture & Codebase Overview

- **Entry Point:** `src/main.rs`
- **Core Components:**
  - `Light` enum (`Dull`, `Bright`) with `print_light_state` function.
  - `Book` struct with lifetime parameter (`'a`) and `display_book` function.
- **Documentation References:**
  - `BASECODE.md`: Detailed structural and behavior breakdown of source code components.
  - `REPO_MAP.json`: JSON structural map of the repository.
