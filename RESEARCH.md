# Phase 0 Research: RWA Token Interfaces on Soroban

Before designing the `RwaCollateral` standard, we conducted a targeted survey of several active RWA projects operating on Soroban. The primary goal was to understand their token models and identify areas where a unified collateral interface would both succeed and face friction. 

## Surveyed Projects

### 1. Invoice Liquidity Network (ILN)
ILN tokenizes factored invoices utilizing an "Invoice NFT" model. In this setup, each distinct invoice has its own corresponding tokenized state managed on-chain.
- **Fit with `RwaCollateral`**: Excellent. Because each invoice can logically act as a unique token contract instance, passing `token: Address` cleanly maps to a specific invoice, and methods like `collateral_value` (representing face value) and `is_redeemable` (based on invoice maturity or default state) directly map to the token's logic.

### 2. Kora Protocol
Kora Protocol functions similarly to ILN by tokenizing invoice financing to provide working capital.
- **Fit with `RwaCollateral`**: Excellent. The underlying asset maps cleanly to a singular real-world claim, making integration trivial. 

### 3. LiquiFact
LiquiFact provides invoice liquidity infrastructure using a typed DTO (Data Transfer Object) state machine approach. Instead of a standard token interface handling everything natively, state requests (e.g., transition to "settled") drive the asset's lifecycle.
- **Fit with `RwaCollateral`**: Good, but requires an adapter. A token contract adopting our trait would need to internally translate `is_redeemable` calls into checks against LiquiFact's internal DTO state trackers. It is feasible, but the implementation is not native out of the box.

### 4. StellarSettle & TradeFlow-Core
These models rely more heavily on permissioned token standards (e.g., ERC-3643 or ERC-7943 uRWA concepts translated to Soroban) and fractionalized fungible vaults (similar to ERC-4626). Here, a single token contract holds pooled value belonging to many different fractional owners.
- **Fit with `RwaCollateral`**: Poor without an intermediate layer. The current signature of `collateral_value(env: Env, token: Address)` queries the *total* value of the contract. It lacks a `user` parameter to identify a specific fractional holder's balance. While the standard could be used by a lending pool calling `token_client.balance(user) * (token_client.collateral_value(token) / token_client.total_supply())`, this shifts complex logic onto the lending pool rather than keeping the trait strictly standard. We honestly flag this as a divergence that the current v1 standard does not serve well.

## Conclusion
The `RwaCollateral` standard fits very cleanly into the 1:1 "Asset = NFT" tokenization models utilized by factoring platforms like ILN and Kora Protocol. However, it struggles to natively capture the state of multi-user vault tokens without forcing lending pools to compute fractional ownership manually. For Phase 1, the standard deliberately targets the NFT-style model as its primary adopter base.
