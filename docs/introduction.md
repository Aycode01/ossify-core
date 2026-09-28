# Introduction

Ossify is a smart contract standard for Real-World Assets (RWAs) on Soroban, providing a unified collateral interface for decentralized finance (DeFi) protocols.

## The Problem

Currently, Soroban's RWA ecosystem is fragmented. Different projects implement their tokenized assets using incompatible interfaces and internal state machines. For example, projects like the Invoice Liquidity Network (ILN) and Kora Protocol tokenize factoring invoices, while LiquiFact uses a typed DTO state machine. Other projects like StellarSettle and TradeFlow-Core rely on fractionalized fungible vaults. 

Because there is no standard way to query an RWA token's value or determine if it is in good standing, DeFi protocols—such as lending and insurance platforms—must write custom, specialized integration code for every single RWA token they wish to support as collateral. That limits how many RWA tokens a single integration can support.

## The Ossify Solution

Ossify introduces a standardized approach to RWA collateral on Soroban through two core components:

1. **The `RwaCollateral` Trait**: A shared Rust trait that RWA token contracts can implement. It defines standard functions (`collateral_value`, `is_redeemable`, and `liquidate`) that any external protocol can call to reliably understand and interact with the asset.
2. **The `RwaRegistry` Contract**: An on-chain directory of RWA tokens that have registered themselves. Consumers use it to check whether a token has declared itself compliant with the standard. Registration only records that the token contract authorized its own listing; it is not an audit of the token's `collateral_value`, `is_redeemable`, or `liquidate` logic.

## Scope of v1

Ossify v1 targets the 1:1 asset-to-token model, meaning one contract per claim, as used by invoice platforms such as Invoice Liquidity Network and Kora Protocol. Fractionalized and vault-style tokens are not natively supported, because `collateral_value` reports the total pool value and leaves fractional ownership to the consumer. See "Known Limitations" in [SPEC.md](../SPEC.md) and the project survey in [RESEARCH.md](../RESEARCH.md).

By implementing the `RwaCollateral` trait and registering with the `RwaRegistry`, token issuers such as ILN and Kora Protocol let consumers that already support Ossify read their assets through the standard interface, instead of writing a bilateral integration per token.
