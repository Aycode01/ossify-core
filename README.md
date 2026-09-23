# Ossify Core

Ossify is a shared collateral standard for tokenized Real-World Assets (RWAs) on the Soroban network. It aims to unify disparate RWA token interfaces (like those for factored invoices, warehouse receipts, and carbon credits) under a single, predictable standard, allowing lending protocols and insurance pools to assess and liquidate any compliant asset seamlessly. For full details on the standard's semantic design, read the [SPEC.md](./SPEC.md).

## Building and Testing
To compile the contracts:
```bash
cargo build --target wasm32-unknown-unknown --release
```

To run the unit tests across the entire workspace:
```bash
cargo test
```

## Repository Layout
- **`contracts/trait/`**: Defines the `RwaCollateral` interface trait.
- **`contracts/registry/`**: A self-registration contract allowing any RWA token to broadcast its compliance with the standard.
- **`contracts/reference-rwa/`**: A mock token implementation demonstrating a correct integration.
- **`contracts/toy-lending-pool/`**: A minimal mock lending pool that utilizes the registry and trait to check token collateral values and issue simulated loans.

## Contributing
Contributors in the Wave program should ensure that every feature, fix, or update is covered by passing unit tests and submitted as a small, focused commit. Please refer to existing pull requests or issues for direction on the current development phase.
