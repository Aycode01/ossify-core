#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

use ossify_registry::{RwaRegistry, RwaRegistryClient};
use ossify_reference_rwa::{MockInvoiceToken, MockInvoiceTokenClient};

#[test]
fn test_toy_lending_pool_borrow() {
    let env = Env::default();
    env.mock_all_auths();
    
    // Deploy registry (though our toy pool doesn't strictly check it yet, it's good practice to set it up)
    let registry_id = env.register_contract(None, RwaRegistry);
    let registry_client = RwaRegistryClient::new(&env, &registry_id);
    
    // Deploy Mock Invoice Token
    let token_id = env.register_contract(None, MockInvoiceToken);
    let _token_client = MockInvoiceTokenClient::new(&env, &token_id);
    
    // Register token in registry
    registry_client.register(&token_id);
    
    // Verify it's registered
    let registered = registry_client.get_registered();
    assert_eq!(registered.len(), 1);
    assert_eq!(registered.get(0).unwrap(), token_id);
    
    // Deploy Toy Lending Pool
    let pool_id = env.register_contract(None, ToyLendingPool);
    let pool_client = ToyLendingPoolClient::new(&env, &pool_id);
    
    // Borrow against it
    let borrow_amount = pool_client.borrow_against(&registry_id, &token_id);
    
    // The mock token returns 1000_0000000. Our pool lends out half.
    assert_eq!(borrow_amount, 500_0000000);
}

#[test]
#[should_panic(expected = "Token is not registered in the RwaRegistry")]
fn test_toy_lending_pool_borrow_unregistered() {
    let env = Env::default();
    
    // Deploy registry
    let registry_id = env.register_contract(None, RwaRegistry);
    
    // Deploy Mock Invoice Token
    let token_id = env.register_contract(None, MockInvoiceToken);
    
    // Do NOT register token in registry
    
    // Deploy Toy Lending Pool
    let pool_id = env.register_contract(None, ToyLendingPool);
    let pool_client = ToyLendingPoolClient::new(&env, &pool_id);
    
    // Attempt to borrow against it, which should panic
    pool_client.borrow_against(&registry_id, &token_id);
}
