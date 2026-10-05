# Contributing to Ossify

We welcome contributions to Ossify! This guide covers how to build, test, and submit your changes.

## Getting Started

1. **Prerequisites**: Ensure you have `rustup` and the Stellar CLI (v25.2.0+) on your `PATH`. The Rust version and the `wasm32v1-none` target are pinned in [`rust-toolchain.toml`](./rust-toolchain.toml), so `rustup toolchain install` gets you the exact toolchain CI uses.
2. **Build**: Run `stellar contract build` to compile the contracts. A plain `cargo build --target wasm32v1-none --release` will fail: `soroban-sdk` requires a build system that runs spec shaking.
3. **Test**: Run `cargo test` to execute all unit tests.

## Claiming a Wave Issue

- If you see an open issue that is part of the Wave program, please comment to claim it before starting work.
- Wait for a maintainer to assign you to avoid duplicated effort.

## Pull Request Expectations

- **One Logical Change per PR**: Keep your PRs focused on a single issue or task.
- **Pass CI**: Ensure all tests pass (`cargo test`) and contracts build (`stellar contract build`) before requesting a review.
- **Format**: Follow standard Rust formatting rules (`cargo fmt`).

## Commit Format

We use conventional commits. Please format your commit messages as:
`type(scope): description`

Examples:
- `feat(registry): add deregister function`
- `docs: add LICENSE`
- `fix(trait): update auth handling`
