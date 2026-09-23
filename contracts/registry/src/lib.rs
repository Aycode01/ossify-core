#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Vec};

#[contracttype]
pub enum DataKey {
    Tokens,
}

#[contract]
pub struct RwaRegistry;

#[contractimpl]
impl RwaRegistry {
    /// Registers a token address as implementing the RwaCollateral trait.
    pub fn register(env: Env, token: Address) {
        let mut tokens: Vec<Address> = env
            .storage()
            .instance()
            .get(&DataKey::Tokens)
            .unwrap_or(Vec::new(&env));
            
        if !tokens.contains(&token) {
            tokens.push_back(token);
            env.storage().instance().set(&DataKey::Tokens, &tokens);
        }
    }

    /// Returns a list of all registered compliant RWA token addresses.
    pub fn get_registered(env: Env) -> Vec<Address> {
        env.storage()
            .instance()
            .get(&DataKey::Tokens)
            .unwrap_or(Vec::new(&env))
    }
}
