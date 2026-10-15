# fountain-stage
Stage extensions for Fountain syntax in rust

## Development

Requires a stable Rust toolchain (pinned via `rust-toolchain.toml`; MSRV 1.85).

```sh
cargo fmt --all                               # format
cargo clippy --all-targets --all-features     # lint (pedantic, warnings fail CI)
cargo test --all-targets && cargo test --doc  # unit, integration and doc tests
cargo doc --no-deps --open                    # API docs
cargo deny check                              # licences/advisories (cargo install cargo-deny)
```

CI (`.github/workflows/ci.yml`) runs all of the above on every push to `main` and on pull requests, with tests on Linux, macOS and Windows plus an MSRV check.
