#![no_std]

mod types;
mod errors;
mod test;

use soroban_sdk::{contract, contractimpl, token, Address, Env};
use types::{Affiliate, DataKey, MerchantConfig};
pub use errors::Error;

#[contract]
pub struct StelliatePayout;

#[contractimpl]
impl StelliatePayout {
    // ── Init ──────────────────────────────────────────────────────────────────

    /// Initialize the contract. Must be called once by the merchant.
    pub fn initialize(
        env: Env,
        merchant: Address,
        usdc_token: Address,
        commission_bps: u32,
    ) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Config) {
            return Err(Error::AlreadyInitialized);
        }
        if commission_bps > 10_000 {
            return Err(Error::InvalidCommission);
        }
        merchant.require_auth();
        env.storage().instance().set(
            &DataKey::Config,
            &MerchantConfig { merchant, commission_bps, usdc_token },
        );
        env.storage().instance().set(&DataKey::EscrowBalance, &0i128);
        Ok(())
    }

    // ── Escrow ────────────────────────────────────────────────────────────────

    /// Merchant deposits USDC into escrow.
    pub fn deposit(env: Env, amount: i128) -> Result<(), Error> {
        let config = Self::config(&env)?;
        config.merchant.require_auth();
        token::Client::new(&env, &config.usdc_token).transfer(
            &config.merchant,
            &env.current_contract_address(),
            &amount,
        );
        Self::update_balance(&env, amount);
        Ok(())
    }

    /// Merchant withdraws from escrow.
    pub fn withdraw(env: Env, amount: i128) -> Result<(), Error> {
        let config = Self::config(&env)?;
        config.merchant.require_auth();
        let balance = Self::balance(env.clone());
        if balance < amount {
            return Err(Error::InsufficientEscrow);
        }
        token::Client::new(&env, &config.usdc_token).transfer(
            &env.current_contract_address(),
            &config.merchant,
            &amount,
        );
        Self::update_balance(&env, -amount);
        Ok(())
    }

    /// Returns current escrow balance.
    pub fn balance(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::EscrowBalance).unwrap_or(0)
    }

    // ── Affiliates ────────────────────────────────────────────────────────────

    /// Merchant registers an affiliate.
    pub fn register_affiliate(env: Env, affiliate: Address) -> Result<(), Error> {
        let config = Self::config(&env)?;
        config.merchant.require_auth();
        env.storage().instance().set(
            &DataKey::Affiliate(affiliate.clone()),
            &Affiliate { address: affiliate, active: true },
        );
        Ok(())
    }

    /// Merchant deactivates an affiliate.
    pub fn deactivate_affiliate(env: Env, affiliate: Address) -> Result<(), Error> {
        let config = Self::config(&env)?;
        config.merchant.require_auth();
        let mut aff = Self::get_affiliate(&env, &affiliate)?;
        aff.active = false;
        env.storage().instance().set(&DataKey::Affiliate(affiliate), &aff);
        Ok(())
    }

    // ── Payout ────────────────────────────────────────────────────────────────

    /// Execute a commission payout for a verified sale.
    /// `sale_amount` is the gross sale value in USDC stroops.
    pub fn pay_commission(env: Env, affiliate: Address, sale_amount: i128) -> Result<(), Error> {
        let config = Self::config(&env)?;
        config.merchant.require_auth();

        let aff = Self::get_affiliate(&env, &affiliate)?;
        if !aff.active {
            return Err(Error::AffiliateInactive);
        }

        let commission = sale_amount * config.commission_bps as i128 / 10_000;
        let balance = Self::balance(env.clone());
        if balance < commission {
            return Err(Error::InsufficientEscrow);
        }

        token::Client::new(&env, &config.usdc_token).transfer(
            &env.current_contract_address(),
            &affiliate,
            &commission,
        );
        Self::update_balance(&env, -commission);
        Ok(())
    }

    // ── Helpers ───────────────────────────────────────────────────────────────

    fn config(env: &Env) -> Result<MerchantConfig, Error> {
        env.storage().instance().get(&DataKey::Config).ok_or(Error::NotInitialized)
    }

    fn get_affiliate(env: &Env, address: &Address) -> Result<Affiliate, Error> {
        env.storage()
            .instance()
            .get(&DataKey::Affiliate(address.clone()))
            .ok_or(Error::AffiliateNotFound)
    }

    fn update_balance(env: &Env, delta: i128) {
        let balance: i128 = env.storage().instance().get(&DataKey::EscrowBalance).unwrap_or(0);
        env.storage().instance().set(&DataKey::EscrowBalance, &(balance + delta));
    }
}
