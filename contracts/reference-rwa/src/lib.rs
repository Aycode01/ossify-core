#![no_std]
use ossify_rwa_trait::RwaCollateral;
use soroban_sdk::{contract, contractimpl, Address, Env, Error};

#[contract]
pub struct MockInvoiceToken;

#[contractimpl]
impl RwaCollateral for MockInvoiceToken {
    fn collateral_value(_env: Env, _token: Address) -> i128 {
        // A mock static value representing a $1,000 invoice (assuming 7 decimals)
        1000_0000000
    }

    fn is_redeemable(_env: Env, _token: Address) -> bool {
        // Mock always redeemable
        true
    }

    fn liquidate(_env: Env, _token: Address, _liquidator: Address) -> Result<(), Error> {
        // Mock successful liquidation
        Ok(())
    }
}
