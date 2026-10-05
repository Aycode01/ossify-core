#![no_std]
use soroban_sdk::{contract, contractevent, contractimpl, Address, Env};
use ossify_rwa_trait::RwaCollateralClient;
use ossify_registry::{RwaRegistry, RwaRegistryClient}; // we actually just need the client or to call it

/// Emitted when the pool lends out against a registered RWA token.
///
/// `data_format = "vec"` keeps the published data a two-element list,
/// `(token, amount_lent)`, matching what `Events::publish` emitted.
#[contractevent(data_format = "vec")]
pub struct Borrow {
    token: Address,
    amount_lent: i128,
}

#[contract]
pub struct ToyLendingPool;

#[contractimpl]
impl ToyLendingPool {
    /// Mocks borrowing against an RWA collateral token.
    /// It queries the token's collateral_value using the RwaCollateral trait.
    pub fn borrow_against(env: Env, registry: Address, token: Address) -> i128 {
        let registry_client = RwaRegistryClient::new(&env, &registry);
        
        // Assert the token is registered in the registry
        let registered_tokens = registry_client.get_registered();
        assert!(registered_tokens.contains(&token), "Token is not registered in the RwaRegistry");
        
        let token_client = RwaCollateralClient::new(&env, &token);
        
        // Assert the token is in good standing
        assert!(token_client.is_redeemable(&token), "Token is not redeemable");
        
        let value = token_client.collateral_value(&token);
        
        // We might lend out 50% of the collateral value
        let amount_lent = value / 2;
        Borrow { token, amount_lent }.publish(&env);
        amount_lent
    }
}
mod test;
