#!/bin/bash
# Scripts to file GitHub issues for remaining Ossify roadmap work

gh issue create \
  --title "feat: create ossify-sdk adapter package" \
  --label "enhancement,Phase 3" \
  --body "### Summary
We need to build the \`ossify-sdk\` to provide adapters for easily wrapping existing RWA tokens with the \`RwaCollateral\` interface.

### Acceptance Criteria
- [ ] Create a new \`ossify-sdk\` crate.
- [ ] Provide wrapper utilities/macros for Soroban tokens.
- [ ] Write integration tests proving an adapter maps seamlessly to the registry.

### Tech Stack
- Rust, Soroban SDK"

gh issue create \
  --title "feat: build second reference consumer (Insurance Pool)" \
  --label "enhancement,Phase 4" \
  --body "### Summary
Currently, we have \`toy-lending-pool\`. We need a second reference consumer, such as an insurance pool, that demonstrates utilizing the \`RwaCollateral\` standard to back insurance policies.

### Acceptance Criteria
- [ ] Create \`contracts/toy-insurance-pool\` crate.
- [ ] Implement logic to query registry and value RWA tokens for insurance liquidity.
- [ ] Cover with unit tests.

### Tech Stack
- Rust, Soroban SDK"

gh issue create \
  --title "feat: integrate fractionalized vaults into the standard" \
  --label "enhancement,Phase 6" \
  --body "### Summary
Currently, \`RwaCollateral\` primarily targets 1:1 asset-to-token models. We need to explore extending the interface or creating a secondary standard to natively support fractionalized/vault-style RWA tokens without forcing consumers to handle fractional math.

### Acceptance Criteria
- [ ] Research and draft an extension to \`SPEC.md\` handling fractional ownership.
- [ ] Discuss with teams like StellarSettle/TradeFlow-Core.
- [ ] Implement prototype vault RWA adapter.

### Tech Stack
- Rust, Soroban SDK"
