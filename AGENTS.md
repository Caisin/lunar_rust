# Repository Guidelines

## Project Structure & Module Organization
- Core library entrypoint is `src/lib/lunar_lib.rs` (crate path is set in `Cargo.toml`).
- Domain modules live under `src/lib/` (for example: `lunar`, `solar`, `eight_char`, `tao`, `foto`, `util`).
- Tests are source-level integration-style files in `src/tests/*_test.rs` and are linked from `src/lib/lunar_lib.rs` via `#[path = "../tests/..."]`.
- Keep new calendar logic close to its domain module and place shared helpers in `src/lib/util/`.

## Build, Test, and Development Commands
- `cargo build` — compile the library and check for type errors.
- `cargo test` — run all test modules wired in `src/lib/lunar_lib.rs`.
- `cargo test lunar_test -- --nocapture` — run a focused test subset while debugging.
- `cargo fmt --all` — format code according to `rustfmt.toml`.
- `cargo clippy --all-targets --all-features` — optional lint pass before opening a PR.

## Coding Style & Naming Conventions
- Follow Rust 2024 edition and existing module layout.
- Formatting is enforced by `rustfmt.toml`: 2-space tabs and ~72 char max width.
- File/module names use `snake_case`; public APIs and traits follow existing patterns (for example `SolarRefHelper`, `LunarRefHelper`).
- Prefer small, focused helper traits and keep calendrical constants/utilities in `util` modules.

## Testing Guidelines
- Add tests in `src/tests/` with file names ending in `_test.rs`.
- Use clear test function names (`lunar_test42`, `test_next_month_edge_case`) and assert full expected strings/dates for regressions.
- Cover edge cases heavily used in this repo: leap months, historical date boundaries (for example 1582-10 transition), and festival resolution.
- After adding a test file, wire it into `src/lib/lunar_lib.rs` with a `#[cfg(test)]` module entry.

## Commit & Pull Request Guidelines
- Current history favors short, direct summaries (for example: `update readme`, `fixed v1.0`). Keep commits atomic and focused.
- Prefer imperative, scoped commit messages (example: `fix: correct lunar leap month conversion`).
- PRs should include: purpose, affected modules, test evidence (`cargo test` output), and sample input/output when behavior changes.
- Link related issues and call out any API-breaking changes explicitly.

## rules
- 方法和关键逻辑需要用备注说明
