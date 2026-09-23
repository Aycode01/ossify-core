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
        token.require_auth();
        let mut tokens: Vec<Address> = env
            .storage()
            .instance()
            .get(&DataKey::Tokens)
            .unwrap_or(Vec::new(&env));
            
        if !tokens.contains(&token) {
            tokens.push_back(token.clone());
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

    /// Deregisters a token address, removing it from the compliant RWA list.
    /// If the token is not currently registered, this function acts as a no-op.
    pub fn deregister(env: Env, token: Address) {
        token.require_auth();
        let mut tokens: Vec<Address> = env
            .storage()
            .instance()
            .get(&DataKey::Tokens)
            .unwrap_or(Vec::new(&env));
            
        if let Some(index) = tokens.first_index_of(&token) {
            tokens.remove(index);
            env.storage().instance().set(&DataKey::Tokens, &tokens);
        }
    }
}
mod test;
