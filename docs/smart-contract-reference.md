# Smart Contract Reference

## 1. `RwaCollateral` Trait (`contracts/trait`)

This standard interface must be implemented by any compliant RWA token.

### `collateral_value`
- **Signature**: `fn collateral_value(env: Env, token: Address) -> i128`
- **Description**: Returns the real-world value of the token. 
- **Who Can Call**: Any external protocol or user.
- **Triggers**: Called when a consumer protocol needs to determine the asset's worth.

### `is_redeemable`
- **Signature**: `fn is_redeemable(env: Env, token: Address) -> bool`
- **Description**: Returns whether the asset is in good standing (e.g., an invoice hasn't defaulted).
- **Who Can Call**: Any external protocol or user.
- **Triggers**: Called during underwriting or periodic health checks.

### `liquidate`
- **Signature**: `fn liquidate(env: Env, token: Address, liquidator: Address) -> Result<(), Error>`
- **Description**: Executes the liquidation process for the token, transferring the claim to the `liquidator`.
- **Who Can Call**: Typically the consumer protocol holding the collateral.
- **Triggers**: Called when a borrower defaults or a position is liquidated.

## 2. `RwaRegistry` (`contracts/registry`)

The central on-chain registry of compliant tokens.

### `register`
- **Signature**: `pub fn register(env: Env, token: Address)`
- **Description**: Adds a token to the registry. Emits a `register` event.
- **Who Can Call**: The `token` contract itself (enforced via `token.require_auth()`).

### `get_registered`
- **Signature**: `pub fn get_registered(env: Env) -> Vec<Address>`
- **Description**: Returns a list of all currently registered RWA token addresses.
- **Who Can Call**: Anyone.

### `deregister`
- **Signature**: `pub fn deregister(env: Env, token: Address)`
- **Description**: Removes a token from the registry. Emits a `deregister` event.
- **Who Can Call**: The `token` contract itself (enforced via `token.require_auth()`).

## 3. `MockInvoiceToken` (`contracts/reference-rwa`)

A reference implementation of a tokenized factoring invoice.

### `collateral_value`
- **Signature**: `fn collateral_value(env: Env, token: Address) -> i128`
- **Description**: Mocks a static value representing a $1,000 invoice (assuming 7 decimals, returns `1000_0000000`).

### `is_redeemable`
- **Signature**: `fn is_redeemable(env: Env, token: Address) -> bool`
- **Description**: Mocks an always-redeemable state (returns `true`).

### `liquidate`
- **Signature**: `fn liquidate(env: Env, token: Address, liquidator: Address) -> Result<(), Error>`
- **Description**: Mocks a successful liquidation (returns `Ok(())`).

### `register_self`
- **Signature**: `pub fn register_self(env: Env, registry: Address)`
- **Description**: A helper function allowing the contract to authorize its own registration in the `RwaRegistry`.
- **Who Can Call**: Anyone, as it strictly registers the contract's own address.

## 4. `ToyLendingPool` (`contracts/toy-lending-pool`)

A reference consumer demonstrating how to integrate Ossify-compliant RWAs.

### `borrow_against`
- **Signature**: `pub fn borrow_against(env: Env, registry: Address, token: Address) -> i128`
- **Description**: Mocks borrowing against an RWA token. Validates the token against the registry, checks redeemability, queries its value, and lends out 50%. Emits a `borrow` event.
- **Who Can Call**: Anyone (in this mock implementation).
