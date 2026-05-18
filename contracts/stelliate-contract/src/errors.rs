use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    Unauthorized = 3,
    AffiliateNotFound = 4,
    AffiliateInactive = 5,
    InsufficientEscrow = 6,
    InvalidCommission = 7,
    InvalidAmount = 8,
}
