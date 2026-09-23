# Ossify Wave Submission Draft

**Project Name**: Ossify
**Repository URL**: [https://github.com/Aycode01/ossify-core](https://github.com/Aycode01/ossify-core)

---

## 1. Project Description
Ossify addresses the problem of fragmented Real-World Asset (RWA) token interfaces on Soroban. Currently, lending pools and insurance protocols must build custom integrations for every RWA token, as projects like Invoice Liquidity Network, Kora Protocol, and LiquiFact all use different on-chain state models. Ossify introduces a unified mechanism: a shared `RwaCollateral` standard trait combined with an on-chain `RwaRegistry`. Built on Soroban using Rust, this architecture is fully tested and deployed to the Stellar testnet. Based on our survey of at least four major RWA projects in the ecosystem, standardizing the 1:1 "Asset = NFT" model creates immediate, seamless interoperability between these tokenized claims and any compliant DeFi lending pool.

## 2. Testnet Contract Deployments (Verified)
The core infrastructure and reference implementations are live on the Stellar Testnet:

- **Registry**: `CB3OM6CSKYXJZUVFJAZWS3IO7JCMSAZSFOCCE35WJLXTUGRI3HJLITY4`
- **Reference RWA (Mock Invoice)**: `CCGFQHBWBLBLRHI33OI7CW6JMKVFAUVB4C372R2KPFZCLQJB2ZGWSYQD`
- **Toy Lending Pool**: `CCGFQHBWBLBLRHI33OI7CW6JMKVFAUVB4C372R2KPFZCLQJB2ZGWSYQD`

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
