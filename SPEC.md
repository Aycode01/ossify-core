# Ossify: Shared Collateral Standard for Tokenized RWAs on Soroban

## Purpose
The `RwaCollateral` standard defines a minimal, shared interface for Real-World Asset (RWA) tokens on the Soroban network. Its goal is to provide lending protocols, insurance pools, and other consumers a unified way to assess the collateral value and liquidation state of any compliant RWA, without needing custom integrations for each individual asset project.

## The `RwaCollateral` Trait
Any RWA contract seeking to be utilized as collateral must implement the following trait. 

```rust
pub trait RwaCollateral {
    fn collateral_value(env: Env, token: Address) -> i128;
    fn is_redeemable(env: Env, token: Address) -> bool;
    fn liquidate(env: Env, token: Address, liquidator: Address) -> Result<(), Error>;
}
```

### 1. `collateral_value(env: Env, token: Address) -> i128`
Returns the current valuation of the RWA token.
- **Parameters**: 
  - `token`: The `Address` of the specific RWA contract instance or asset. For singular NFT-style assets (e.g., an individual factored invoice), this is the contract address of the asset itself.
- **Returns**: An `i128` representing the token's value in a standardized base unit (e.g., using 7 decimal places for a stablecoin peg).
- **Semantics**: The RWA contract is responsible for accurately reporting its internal value or fetching it via an oracle. Lending protocols will use this value to calculate borrowing power and collateralization ratios.

### 2. `is_redeemable(env: Env, token: Address) -> bool`
Checks if the asset is in good standing and available for redemption.
- **Parameters**:
  - `token`: The `Address` of the RWA token being queried.
- **Returns**: A boolean indicating whether the asset can currently be redeemed or if it has expired, defaulted, or been locked.
- **Semantics**: A value of `false` implies the RWA is impaired or illiquid. Lending pools should generally reject collateral if `is_redeemable` returns `false`, and may choose to initiate liquidation on existing loans.

### 3. `liquidate(env: Env, token: Address, liquidator: Address) -> Result<(), Error>`
Executes a liquidation, transferring the underlying claim or ownership of the RWA to a designated liquidator.
- **Parameters**:
  - `token`: The `Address` of the RWA token.
  - `liquidator`: The `Address` that should receive ownership or the underlying real-world claim to the asset.
- **Returns**: `Ok(())` on a successful liquidation, or a Soroban `Error` if the process fails (e.g., if the token is not currently eligible for liquidation, or the caller lacks authorization).
- **Semantics**: The caller of this function is typically a lending pool executing a default. Implementers must ensure that this function adheres strictly to their platform's legal and on-chain liquidation policies.

## The RWA Registry
Consumers can query the `RwaRegistry` contract to verify if an RWA contract is officially compliant with the `RwaCollateral` standard. This prevents spoofing and allows protocols to automatically onboard newly compliant assets.

## Adoption and Compatibility
The `token: Address` parameter is designed to treat the RWA contract itself as the distinct asset, matching the architectural patterns of leading invoice factoring projects where `1 Contract = 1 Invoice`. For platforms using fractionalized fungible vaults, the `token` parameter can be used to pass a specific vault's address, though such platforms should ensure they report the total available pool value, requiring consumers to measure their fractional share independently.
