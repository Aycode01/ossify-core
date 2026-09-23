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
    pub fn borrow_against(env: Env, token: Address) -> i128 {
        // In a real pool, we would check if the token is registered in the registry
        // by making a cross-contract call to the registry address.
        // For this toy example, we just invoke the trait method directly on the token.
        
        let client = RwaCollateralClient::new(&env, &token);
        
        // Assert the token is in good standing
        assert!(client.is_redeemable(&token), "Token is not redeemable");
        
        let value = client.collateral_value(&token);
        
        // We might lend out 50% of the collateral value
        value / 2
    }
}
mod test;
