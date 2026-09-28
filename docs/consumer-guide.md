# Consumer Guide

This guide is for decentralized applications (DeFi), such as lending pools, AMMs, or insurance protocols, that want to integrate any Ossify-compliant Real-World Asset (RWA).

## Why Integrate Ossify?

Without Ossify, your protocol must implement a custom interface and state-checking logic for every tokenized RWA. By supporting Ossify, you write your integration logic once and can accept any registered token.

## Scope

Ossify v1 targets 1:1 asset-to-token models, where each registered contract represents one claim and `collateral_value` returns that claim's value. For fractionalized or vault-style tokens, `collateral_value` reports the total pool, so your protocol has to measure a holder's share itself. See "Known Limitations" in [SPEC.md](../SPEC.md) and the survey in [RESEARCH.md](../RESEARCH.md).

## Step 1: Add Dependencies

Add the registry and trait clients to your `Cargo.toml`.

```toml
[dependencies]
ossify-registry = "0.1.0"
ossify-rwa-trait = "0.1.0"
```

## Step 2: Verify the Token

Before interacting with an RWA, verify it is registered in the official `RwaRegistry`. You can either store the registry address in your contract or pass it as an argument.

```rust
use ossify_registry::RwaRegistryClient;

// Ensure token is registered
let registry_client = RwaRegistryClient::new(&env, &registry_address);
let registered_tokens = registry_client.get_registered();
assert!(registered_tokens.contains(&token_address), "Token is not Ossify-compliant");
```

## Step 3: Check Token Health

Call `is_redeemable` to verify that the underlying asset is in good standing. This protects your protocol from accepting defaulted or locked collateral.

```rust
use ossify_rwa_trait::RwaCollateralClient;

let token_client = RwaCollateralClient::new(&env, &token_address);
assert!(token_client.is_redeemable(&token_address), "Token is defaulted or not redeemable");
```

## Step 4: Value the Collateral

Query the exact value of the asset. Because the value represents a real-world claim (e.g., $1,000 USD), your protocol can use it as the input to its own Loan-to-Value (LTV) calculation. The registry does not verify the reported value, so the risk policy, haircut, and LTV ceiling remain your protocol's decision.

```rust
let collateral_value = token_client.collateral_value(&token_address);

// Example: Lend out 50% of the collateral value
let borrow_amount = collateral_value / 2;
```

## Liquidations

If the position becomes undercollateralized or the borrower defaults, your protocol can call the standardized `liquidate` function to claim the underlying asset.

```rust
// Liquidate and transfer the claim to the liquidator address
token_client.liquidate(&token_address, &liquidator_address);
```
