#![cfg(test)]

use super::*;
use ossify_rwa_trait::RwaCollateralClient;
use ossify_registry::{RwaRegistry, RwaRegistryClient};
use soroban_sdk::{testutils::Address as _, Address, Env};

/// Deploys the reference token and a registry into a fresh test environment.
/// `mock_all_auths` lets the token authorize its own registration.
fn setup() -> (Env, Address, Address) {
    let env = Env::default();
    env.mock_all_auths();

    let registry_id = env.register(RwaRegistry, ());
    let token_id = env.register(MockInvoiceToken, ());
    (env, registry_id, token_id)
}

#[test]
fn test_collateral_value_returns_mock_value() {
    let (env, _registry_id, token_id) = setup();
    let token_client = MockInvoiceTokenClient::new(&env, &token_id);

    // The documented mock value: a $1,000 invoice in 7 decimals.
    assert_eq!(token_client.collateral_value(&token_id), 1000_0000000);
}

#[test]
fn test_is_redeemable_returns_true() {
    let (env, _registry_id, token_id) = setup();
    let token_client = MockInvoiceTokenClient::new(&env, &token_id);

    assert!(token_client.is_redeemable(&token_id));
}

#[test]
fn test_liquidate_succeeds() {
    let (env, _registry_id, token_id) = setup();
    let liquidator = Address::generate(&env);

    // Call the trait method directly so the `Result` is observable: the
    // generated client flattens it to `()`.
    let result = <MockInvoiceToken as RwaCollateral>::liquidate(env, token_id, liquidator);

    assert_eq!(result, Ok(()));
}

#[test]
fn test_register_self_registers_token_in_registry() {
    let (env, registry_id, token_id) = setup();
    let registry_client = RwaRegistryClient::new(&env, &registry_id);
    let token_client = MockInvoiceTokenClient::new(&env, &token_id);

    assert!(registry_client.get_registered().is_empty());

    token_client.register_self(&registry_id);

    let registered = registry_client.get_registered();
    assert_eq!(registered.len(), 1);
    assert!(registered.contains(&token_id));
    assert_eq!(registered.get(0).unwrap(), token_id);
}

/// Exercises the `RwaCollateralClient` path a consumer protocol uses, against
/// the deployed token, exactly as the consumer guide describes.
#[test]
fn test_consumer_can_call_token_through_trait_client() {
    let (env, registry_id, token_id) = setup();
    let token_client = MockInvoiceTokenClient::new(&env, &token_id);
    token_client.register_self(&registry_id);

    let registry_client = RwaRegistryClient::new(&env, &registry_id);
    assert!(registry_client.get_registered().contains(&token_id));

    let collateral_client = RwaCollateralClient::new(&env, &token_id);
    assert!(collateral_client.is_redeemable(&token_id));

    let value = collateral_client.collateral_value(&token_id);
    assert_eq!(value, 1000_0000000);

    let liquidator = Address::generate(&env);
    collateral_client.liquidate(&token_id, &liquidator);
}
