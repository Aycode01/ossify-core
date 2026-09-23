# Adopter Guide

This guide is written for Real-World Asset token issuers (e.g., platforms like Invoice Liquidity Network or Kora Protocol) wanting to make their tokens Ossify-compliant. By adopting the `RwaCollateral` standard, your tokens instantly become usable across the Soroban DeFi ecosystem without requiring custom integrations.

## Step 1: Add the Trait Dependency

First, add the `ossify-rwa-trait` dependency to your token contract's `Cargo.toml`.

```toml
[dependencies]
ossify-rwa-trait = "0.1.0"
```

## Step 2: Implement the `RwaCollateral` Trait

In your token contract, implement the trait for your contract's main struct. You must define three functions: `collateral_value`, `is_redeemable`, and `liquidate`.

```rust
use ossify_rwa_trait::RwaCollateral;
use soroban_sdk::{contractimpl, Address, Env, Error};

#[contractimpl]
impl RwaCollateral for MyInvoiceToken {
    fn collateral_value(env: Env, token: Address) -> i128 {
        // Return the face value or discounted value of your asset.
        // Example: read from your token's internal state.
        Self::get_face_value(&env)
    }

    fn is_redeemable(env: Env, token: Address) -> bool {
        // Return true if the asset is in good standing.
        // Example: check if the invoice is past due or defaulted.
        Self::get_status(&env) != Status::Defaulted
    }

    fn liquidate(env: Env, token: Address, liquidator: Address) -> Result<(), Error> {
        // Implement the logic to transfer ownership or claim rights to the liquidator.
        Self::transfer_claim(&env, liquidator);
        Ok(())
    }
}
```

## Step 3: Enable Registration

Because the `RwaRegistry` ensures security by requiring the token contract to authorize its own registration (`token.require_auth()`), your contract must provide a way to invoke the registry.

You can add a helper function for your admin to trigger the registration:

```rust
use ossify_registry::RwaRegistryClient;

#[contractimpl]
impl MyInvoiceToken {
    pub fn register_in_ossify(env: Env, registry: Address) {
        // (Optional) Require your admin's auth here to restrict who can register the token.
        // admin.require_auth();
        
        let registry_client = RwaRegistryClient::new(&env, &registry);
        registry_client.register(&env.current_contract_address());
    }
}
```

## Step 4: Deploy and Register

Deploy your token to the Stellar network as usual. Once deployed, invoke your `register_in_ossify` function, passing the official Ossify `RwaRegistry` contract ID.

Your token is now Ossify-compliant and can be utilized by any consumer protocol!
