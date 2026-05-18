use soroban_sdk::{contracttype, Address};

#[contracttype]
#[derive(Clone)]
pub struct MerchantConfig {
    pub merchant: Address,
    pub commission_bps: u32, // basis points, e.g. 1000 = 10%
    pub usdc_token: Address,
}

#[contracttype]
#[derive(Clone)]
pub struct Affiliate {
    pub address: Address,
    pub active: bool,
}

#[contracttype]
pub enum DataKey {
    Config,
    Affiliate(Address),
    EscrowBalance,
}
