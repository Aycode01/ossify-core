#!/bin/bash
# Scripts to file GitHub issues for remaining Ossify roadmap work
#
# Review before running: this only files issues, it does not label or assign them.
# Requires the `gh` CLI, authenticated, with repo scope on the target repository.

gh issue create \
  --title "feat: create ossify-sdk adapter package" \
  --body "### Summary
We need to build the \`ossify-sdk\` to provide adapters for easily wrapping existing RWA tokens with the \`RwaCollateral\` interface.

### Acceptance Criteria
- [ ] Create a new \`ossify-sdk\` crate.
- [ ] Provide wrapper utilities/macros for Soroban tokens.
- [ ] Write integration tests proving an adapter registers correctly with the registry.

### Tech Stack
- Rust, Soroban SDK"

gh issue create \
  --title "feat: build second reference consumer (Insurance Pool)" \
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
  --body "### Summary
Currently, \`RwaCollateral\` primarily targets 1:1 asset-to-token models. We need to explore extending the interface or creating a secondary standard to natively support fractionalized/vault-style RWA tokens without forcing consumers to handle fractional math.

### Acceptance Criteria
- [ ] Research and draft an extension to \`SPEC.md\` handling fractional ownership.
- [ ] Discuss with teams like StellarSettle/TradeFlow-Core.
- [ ] Implement prototype vault RWA adapter.

### Tech Stack
- Rust, Soroban SDK"

gh issue create \
  --title "feat(registry): add paginated get_registered for large token lists" \
  --body "### Summary
\`RwaRegistry::get_registered\` returns the full \`Vec<Address>\` in a single call. As the registry grows, that response becomes unbounded, which is a practical limit on Soroban due to the read footprint of a single invocation. We need a cursor-based paginated view that mirrors how indexers already walk large collections.

### Acceptance Criteria
- [ ] Add \`get_registered_page(cursor: Option<u32>, limit: u32)\` returning a page plus a next cursor.
- [ ] Document the expected page size bound and the error returned when \`limit\` is zero or above the maximum.
- [ ] Keep \`get_registered\` for backward compatibility, with a doc comment noting it is not for large registries.
- [ ] Add unit tests covering the first page, a middle page, the last page, and a cursor that yields an empty page.

### Tech Stack
- Rust, Soroban SDK"

gh issue create \
  --title "feat(registry): add get_collateral_info view for indexers" \
  --body "### Summary
Indexers currently have to make three separate calls per token (\`is_registered\`, \`collateral_value\`, \`is_redeemable\`) to build a view of the ecosystem. A single aggregate view makes ingestion cheaper and gives consumers a stable struct to read.

### Acceptance Criteria
- [ ] Add a \`#[contracttype] CollateralInfo\` struct with the token address, collateral value, and redeemable flag.
- [ ] Add \`get_collateral_info(token) -> Option<CollateralInfo>\` that returns \`None\` for unregistered tokens.
- [ ] Add the necessary events so indexers can track state changes without polling.
- [ ] Add unit tests for a registered token, an unregistered token, and a token whose value has changed since registration.

### Tech Stack
- Rust, Soroban SDK"

gh issue create \
  --title "test(reference-rwa): cover liquidation and redemption logic" \
  --body "### Summary
\`MockInvoiceToken\` returns a hardcoded \`1000_0000000\`, \`true\`, and \`Ok(())\`. The toy lending pool has tests, but the reference RWA itself has none, so the sample implementation that adopters are expected to copy is unverified.

### Acceptance Criteria
- [ ] Add a test asserting \`collateral_value\`, \`is_redeemable\`, and \`liquidate\` return the documented values for a fresh token.
- [ ] Add a test that \`register_self\` registers the token's own address and emits the registry event.
- [ ] Add a test covering the case where liquidation is rejected, once the mock supports a non-redeemable state.
- [ ] Update \`docs/adopter-guide.md\` if the reference token's behavior changes.

### Tech Stack
- Rust, Soroban SDK, \`soroban-sdk::testutils\`"

gh issue create \
  --title "feat: add second reference token modelling a warehouse receipt" \
  --body "### Summary
The only reference token models a factored invoice. A warehouse receipt exercises different lifecycle states, such as expiry and default, which gives consumers a second shape to integrate against and makes the standard's scope clearer.

### Acceptance Criteria
- [ ] Create \`contracts/reference-warehouse-receipt\`.
- [ ] Implement \`RwaCollateral\` with a non-constant \`collateral_value\` read from contract storage.
- [ ] Model expiry and default so \`is_redeemable\` returns \`false\` after a maturity date.
- [ ] Register it against the testnet registry and add unit tests for both the healthy and defaulted states.

### Tech Stack
- Rust, Soroban SDK"

gh issue create \
  --title "spike: fractional-share adapter for vault-style RWA tokens" \
  --body "### Summary
\`SPEC.md\` flags that v1 does not serve fractionalized vault tokens well, and \`RESEARCH.md\` notes the current workaround pushes fractional math onto the lending pool (\`balance(user) * collateral_value / total_supply\`). This issue is a scoped spike to measure that workaround against a real vault, before deciding whether to extend the trait.

### Acceptance Criteria
- [ ] Build a throwaway vault-style reference token holding pooled value.
- [ ] Implement the workaround above in a consumer and document the rounding and stale-price failure modes.
- [ ] Write up whether a \`user\` parameter or a separate \`FractionalCollateral\` trait is the better fit, with a recommendation.
- [ ] Do not commit the prototype to the standard; the outcome is a design note in \`RESEARCH.md\`.

### Tech Stack
- Rust, Soroban SDK, ERC-4626-style vault modeling"

gh issue create \
  --title "test(registry): add property-based tests for register and deregister" \
  --body "### Summary
The registry has two example-style unit tests. Its invariants, that a token appears at most once and that deregister is idempotent, are currently only checked by hand-written cases.

### Acceptance Criteria
- [ ] Add a randomized test that registers and deregisters an arbitrary sequence of tokens.
- [ ] Assert after every step that no token is duplicated and that membership matches the expected set.
- [ ] Assert \`deregister\` on a never-registered token is a no-op and emits no event.
- [ ] Keep the test deterministic with a fixed seed so CI failures are reproducible.

### Tech Stack
- Rust, \`proptest\` or \`soroban-sdk::testutils\` sequences"

gh issue create \
  --title "docs: add a worked end-to-end testnet example" \
  --body "### Summary
The guides describe each step in isolation. A reader has to assemble the deploy, register, and borrow sequence themselves, and the current testnet contract IDs are not written down anywhere in \`docs/\`.

### Acceptance Criteria
- [ ] Add a \`docs/walkthrough.md\` covering \`stellar contract build\`, deploy, \`register_self\`, and \`borrow_against\` end to end, with real command output.
- [ ] Record the current testnet contract IDs and the \`stellar contract info interface\` command used to verify them.
- [ ] Show the unauthorized-registration failure case and the error a consumer sees when a token is not registered.
- [ ] Note which values are hardcoded mocks so readers do not treat the output as a live pricing example.

### Tech Stack
- Markdown, Stellar CLI"
