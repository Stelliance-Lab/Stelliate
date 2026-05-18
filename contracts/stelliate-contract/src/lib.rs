#![no_std]

mod types;
mod errors;
mod test;

use soroban_sdk::{contract, contractimpl, token, Address, Env};
use types::{Affiliate, DataKey, MerchantConfig};
pub use errors::Error;

// TTL constants (in ledgers; ~5s/ledger on Stellar)
const DAY_IN_LEDGERS: u32 = 17_280;
const INSTANCE_TTL_BUMP: u32 = 30 * DAY_IN_LEDGERS;   // 30 days
const INSTANCE_TTL_THRESHOLD: u32 = 7 * DAY_IN_LEDGERS; // bump when < 7 days remain

#[contract]
pub struct StelliatePayout;

#[contractimpl]
impl StelliatePayout {
    // ── Init ──────────────────────────────────────────────────────────────────

    /// Initialize the contract. Must be called once by the merchant.
    /// `oracle` is the trusted off-chain address that triggers commission payouts.
    pub fn initialize(
        env: Env,
        merchant: Address,
        oracle: Address,
        usdc_token: Address,
        commission_bps: u32,
    ) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Config) {
            return Err(Error::AlreadyInitialized);
        }
        if commission_bps == 0 || commission_bps > 10_000 {
            return Err(Error::InvalidCommission);
        }
        merchant.require_auth();
        env.storage().instance().set(
            &DataKey::Config,
            &MerchantConfig { merchant, oracle, commission_bps, usdc_token },
        );
        env.storage().instance().set(&DataKey::EscrowBalance, &0i128);
        Self::bump_ttl(&env);
        Ok(())
    }

    // ── Escrow ────────────────────────────────────────────────────────────────

    /// Merchant deposits USDC into escrow.
    pub fn deposit(env: Env, amount: i128) -> Result<(), Error> {
        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }
        let config = Self::config(&env)?;
        config.merchant.require_auth();
        token::Client::new(&env, &config.usdc_token).transfer(
            &config.merchant,
            &env.current_contract_address(),
            &amount,
        );
        Self::update_balance(&env, amount);
        Self::bump_ttl(&env);
        Ok(())
    }

    /// Merchant withdraws from escrow.
    pub fn withdraw(env: Env, amount: i128) -> Result<(), Error> {
        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }
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
        Self::bump_ttl(&env);
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
        env.storage()
            .instance()
            .set(&DataKey::Affiliate(affiliate), &Affiliate { active: true });
        Self::bump_ttl(&env);
        Ok(())
    }

    /// Merchant deactivates an affiliate.
    pub fn deactivate_affiliate(env: Env, affiliate: Address) -> Result<(), Error> {
        let config = Self::config(&env)?;
        config.merchant.require_auth();
        // Ensure affiliate exists before deactivating
        Self::get_affiliate(&env, &affiliate)?;
        env.storage()
            .instance()
            .set(&DataKey::Affiliate(affiliate), &Affiliate { active: false });
        Self::bump_ttl(&env);
        Ok(())
    }

    // ── Payout ────────────────────────────────────────────────────────────────

    /// Execute a commission payout for a verified sale.
    /// Must be called by the oracle (trusted off-chain relayer).
    /// `sale_amount` is the gross sale value in USDC stroops.
    pub fn pay_commission(env: Env, affiliate: Address, sale_amount: i128) -> Result<(), Error> {
        if sale_amount <= 0 {
            return Err(Error::InvalidAmount);
        }
        let config = Self::config(&env)?;
        config.oracle.require_auth();

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
        Self::bump_ttl(&env);
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

    fn bump_ttl(env: &Env) {
        env.storage().instance().extend_ttl(INSTANCE_TTL_THRESHOLD, INSTANCE_TTL_BUMP);
    }
}
