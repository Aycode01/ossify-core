# Ossify Core

[![Build Status](https://github.com/Aycode01/ossify-core/actions/workflows/ci.yml/badge.svg)](https://github.com/Aycode01/ossify-core/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Ossify is a shared collateral standard for tokenized Real-World Assets (RWAs) on the Soroban network. It aims to unify disparate RWA token interfaces (like those for factored invoices, warehouse receipts, and carbon credits) under a single, predictable standard, allowing lending protocols and insurance pools to assess and liquidate any compliant asset seamlessly. For full details on the standard's semantic design, read the [SPEC.md](./SPEC.md).

**Maintainer**: Omitogun Ayobami ([@Aycode01](https://github.com/Aycode01))  
**Community**: [Join our Discord](https://discord.gg/ossify)

## Architecture & Layout
- **`contracts/trait/`**: Defines the `RwaCollateral` interface trait.
- **`contracts/registry/`**: A self-registration contract allowing any RWA token to broadcast its compliance with the standard.
- **`contracts/reference-rwa/`**: A mock token implementation demonstrating a correct integration.
- **`contracts/toy-lending-pool/`**: A minimal mock lending pool that utilizes the registry and trait to check token collateral values and issue simulated loans.

## Quick Start
To compile the contracts:
```bash
cargo build --target wasm32v1-none --release
```

To run the unit tests across the entire workspace:
```bash
cargo test
```

## Contributing
We welcome contributions! Please see our [CONTRIBUTING.md](./CONTRIBUTING.md) for full details on how to get started, claim issues, and submit PRs.

## Contributors
<a href="https://github.com/Aycode01/ossify-core/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=Aycode01/ossify-core" />
</a>
