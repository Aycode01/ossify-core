# Ossify Wave Submission Draft

**Project Name**: Ossify
**Repository URL**: [https://github.com/Aycode01/ossify-core](https://github.com/Aycode01/ossify-core)

---

## 1. Project Description
Real-World Asset (RWA) projects on Soroban each use a different on-chain state model, so lending pools and insurance protocols have to build a custom integration for every tokenized claim they want to accept. Ossify targets that fragmentation with two pieces: a shared `RwaCollateral` trait that any RWA token can implement, and an on-chain `RwaRegistry` where token contracts self-register, so consumers can discover compliant tokens and read their collateral value, redemption status, and liquidation path through one interface. Both are written in Rust on Soroban, covered by unit tests, and deployed to the Stellar testnet. The scope of v1 is the 1:1 asset-to-token model, meaning one contract per claim, as used by invoice platforms such as Invoice Liquidity Network and Kora Protocol (see [RESEARCH.md](./RESEARCH.md)). Fractionalized and vault-style tokens such as StellarSettle and TradeFlow-Core are not natively supported: `collateral_value` reports the whole pool there, leaving consumers to work out fractional shares themselves. See "Known Limitations" in [SPEC.md](./SPEC.md). Registration also only records that a token authorized its own listing; it is not an audit of that token's valuation or liquidation logic.

## 2. Testnet Contract Deployments (Verified)
The core infrastructure and reference implementations are live on the Stellar Testnet:

- **Registry**: `CB3OM6CSKYXJZUVFJAZWS3IO7JCMSAZSFOCCE35WJLXTUGRI3HJLITY4`
- **Reference RWA (Mock Invoice)**: `CCGFQHBWBLBLRHI33OI7CW6JMKVFAUVB4C372R2KPFZCLQJB2ZGWSYQD`
- **Toy Lending Pool**: `CBFB44ZY6KZP7A3FWPZ5TMMJO3EZO2JWAM5H6IF5P5VPMBEHVEHLOMON`

*(All deployments can be verified on Stellar Expert using the IDs above).*

## 3. Documentation
The full documentation site content is available in the `docs/` directory of the repository, covering:
- Protocol mechanics and standard lifecycle
- Smart contract reference
- Adopter Guide for RWA issuers
- Consumer Guide for DeFi protocols

## 4. Planned Issues (Roadmap)
We have identified and filed the following roadmap items as GitHub issues to guide ongoing development:
- **Architecture**: `feat: add ossify-sdk adapters` for platforms using alternative DTO models (e.g., LiquiFact).
- **Architecture**: `feat: fractional vault support` to extend the standard for ERC-4626 style multi-owner RWAs.
- **Consumers**: `feat: add a second reference consumer` demonstrating a basic AMM or insurance pool integration.

## 5. Demo Video
*(MANUAL STEP: Insert link to screen recording showing registry interaction, collateral querying, and the unauthorized registration failure case).*
