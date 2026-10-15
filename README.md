# fountain-stage
Stage extensions for Fountain syntax in rust

## Layout

This is a Cargo workspace with two crates:

| Crate | Path | What it is |
| --- | --- | --- |
| `fountain-stage` | `crates/fountain-stage` | Library: parsing and handling of the Fountain stage extensions |
| `fountain-stage-cli` | `crates/fountain-stage-cli` | Command-line tool (binary `fountain-stage`) built on the library |

Run the CLI with `cargo run -p fountain-stage-cli -- --help`.

## Development

Requires a stable Rust toolchain (pinned via `rust-toolchain.toml`; MSRV 1.85).

```sh
cargo fmt --all                               # format
cargo clippy --workspace --all-targets        # lint (pedantic, warnings fail CI)
cargo test --workspace                        # unit, integration and doc tests
cargo doc --workspace --no-deps --open        # API docs
cargo deny check                              # licences/advisories (cargo install cargo-deny)
```

CI (`.github/workflows/ci.yml`) runs all of the above on every push to `main` and on pull requests, with tests on Linux, macOS and Windows plus an MSRV check.
