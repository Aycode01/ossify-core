# Contributing to Ossify

We welcome contributions to Ossify! This guide covers how to build, test, and submit your changes.

## Getting Started

1. **Prerequisites**: Ensure you have Rust installed along with the `wasm32v1-none` target.
2. **Build**: Run `cargo build` to compile the contracts.
3. **Test**: Run `cargo test` to execute all unit tests.

## Claiming a Wave Issue

- If you see an open issue labeled for the Wave program, please comment to claim it before starting work.
- Wait for a maintainer to assign you to avoid duplicated effort.

## Pull Request Expectations

- **One Logical Change per PR**: Keep your PRs focused on a single issue or task.
- **Pass CI**: Ensure all tests pass (`cargo test`) before requesting a review.
- **Format**: Follow standard Rust formatting rules (`cargo fmt`).

## Commit Format

We use conventional commits. Please format your commit messages as:
`type(scope): description`

Examples:
- `feat(registry): add deregister function`
- `docs: add LICENSE`
- `fix(trait): update auth handling`
