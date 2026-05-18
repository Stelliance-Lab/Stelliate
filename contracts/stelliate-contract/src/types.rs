use soroban_sdk::{contracttype, Address};

#[contracttype]
#[derive(Clone)]
pub struct MerchantConfig {
    pub merchant: Address,
    pub oracle: Address,   // trusted address allowed to trigger payouts
    pub commission_bps: u32,
    pub usdc_token: Address,
}

/// Affiliate is just a bool (active flag); address is the storage key.
#[contracttype]
#[derive(Clone)]
pub struct Affiliate {
    pub active: bool,
}

#[contracttype]
pub enum DataKey {
    Config,
    Affiliate(Address),
    EscrowBalance,
}
