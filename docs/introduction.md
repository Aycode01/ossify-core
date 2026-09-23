# Introduction

Ossify is a smart contract standard for Real-World Assets (RWAs) on Soroban, providing a unified collateral interface for decentralized finance (DeFi) protocols.

## The Problem

Currently, Soroban's RWA ecosystem is fragmented. Different projects implement their tokenized assets using incompatible interfaces and internal state machines. For example, projects like the Invoice Liquidity Network (ILN) and Kora Protocol tokenize factoring invoices, while LiquiFact uses a typed DTO state machine. Other projects like StellarSettle and TradeFlow-Core rely on fractionalized fungible vaults. 

Because there is no standard way to query an RWA token's value or determine if it is in good standing, DeFi protocols—such as lending and insurance platforms—must write custom, specialized integration code for every single RWA token they wish to support as collateral. This severely limits liquidity and composability.

## The Ossify Solution

Ossify introduces a standardized approach to RWA collateral on Soroban through two core components:

1. **The `RwaCollateral` Trait**: A shared Rust trait that RWA token contracts can implement. It defines standard functions (`collateral_value`, `is_redeemable`, and `liquidate`) that any external protocol can call to reliably understand and interact with the asset.
2. **The `RwaRegistry` Contract**: An on-chain registry of compliant RWA tokens. It serves as a trusted source of truth for lending pools and other DeFi consumers to verify if a given token implements the standard and is currently active.

By implementing the `RwaCollateral` trait and registering with the `RwaRegistry`, token issuers like ILN and Kora Protocol can instantly make their RWAs compatible with any Ossify-compliant lending pool, without needing bilateral custom integrations.
