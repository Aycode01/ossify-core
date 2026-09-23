#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env};
use ossify_rwa_trait::RwaCollateralClient;
use ossify_registry::{RwaRegistry, RwaRegistryClient}; // we actually just need the client or to call it

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
        env.events().publish((soroban_sdk::symbol_short!("borrow"),), (token, amount_lent));
        amount_lent
    }
}
mod test;
