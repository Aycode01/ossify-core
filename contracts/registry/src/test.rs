#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn test_registry_register_and_deregister() {
    let env = Env::default();
    env.mock_all_auths();
    
    let registry_id = env.register_contract(None, RwaRegistry);
    let client = RwaRegistryClient::new(&env, &registry_id);
    
    let token1 = Address::generate(&env);
    let token2 = Address::generate(&env);

    // Initial state
    assert_eq!(client.get_registered().len(), 0);

    // Register a new token
    client.register(&token1);
    let registered = client.get_registered();
    assert_eq!(registered.len(), 1);
    assert_eq!(registered.get(0).unwrap(), token1.clone());

    // Attempt to register a duplicate
    client.register(&token1);
    let registered = client.get_registered();
    assert_eq!(registered.len(), 1);

    // Register a second token
    client.register(&token2);
    let registered = client.get_registered();
    assert_eq!(registered.len(), 2);

    // Deregister an existing token
    client.deregister(&token1);
    let registered = client.get_registered();
    assert_eq!(registered.len(), 1);
    assert_eq!(registered.get(0).unwrap(), token2.clone());

    // Deregister a token that was never registered (or already deregistered)
    client.deregister(&token1);
    let registered = client.get_registered();
    assert_eq!(registered.len(), 1);
    assert_eq!(registered.get(0).unwrap(), token2.clone());
}

#[test]
#[should_panic(expected = "HostError: Error(Auth, InvalidAction)")]
fn test_registry_register_unauthorized() {
    let env = Env::default();
    // Do NOT mock auth here to verify it panics
    
    let registry_id = env.register_contract(None, RwaRegistry);
    let client = RwaRegistryClient::new(&env, &registry_id);
    
    let token = Address::generate(&env);
    
    // Should panic due to missing auth
    client.register(&token);
}
