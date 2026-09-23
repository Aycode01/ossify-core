#![no_std]
use ossify_rwa_trait::RwaCollateral;
use soroban_sdk::{contract, contractimpl, Address, Env, Error};

#[contract]
pub struct MockInvoiceToken;

#[contractimpl]
impl MockInvoiceToken {
    pub fn register_self(env: Env, registry: Address) {
        let registry_client = ossify_registry::RwaRegistryClient::new(&env, &registry);
        registry_client.register(&env.current_contract_address());
    }
}

#[contractimpl]
impl RwaCollateral for MockInvoiceToken {
    fn collateral_value(_env: Env, _token: Address) -> i128 {
        1000_0000000
    }

    fn is_redeemable(_env: Env, _token: Address) -> bool {
        true
    }

    fn liquidate(_env: Env, _token: Address, _liquidator: Address) -> Result<(), Error> {
        Ok(())
    }
}
