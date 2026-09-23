#![no_std]

use soroban_sdk::{contractclient, Address, Env, Error};

/// The RwaCollateral trait defines a standard interface for real-world asset (RWA) tokens
/// to report their status and handle liquidations, allowing lending protocols to integrate
/// with any compliant RWA token seamlessly.
#[contractclient(name = "RwaCollateralClient")]
pub trait RwaCollateral {
    /// Returns the current collateral value of the token.
    /// The token parameter refers to the specific RWA token contract instance.
    fn collateral_value(env: Env, token: Address) -> i128;

    /// Returns whether the asset is currently in good standing and can be redeemed
    /// by the token holder for the underlying real-world asset or payment.
    fn is_redeemable(env: Env, token: Address) -> bool;

    /// Liquidates the RWA collateral. This is called by a lending pool or liquidator
    /// when the borrower defaults. The token should transfer ownership or underlying
    /// claims to the `liquidator`.
    /// Returns an error if the liquidation fails.
    fn liquidate(env: Env, token: Address, liquidator: Address) -> Result<(), Error>;
}
