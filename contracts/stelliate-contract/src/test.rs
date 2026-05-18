#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, token, Address, Env};

fn setup() -> (Env, Address, Address, Address, Address) {
    let env = Env::default();
    env.mock_all_auths();

    let merchant = Address::generate(&env);
    let oracle = Address::generate(&env);

    let usdc_admin = Address::generate(&env);
    let usdc_id = env.register_stellar_asset_contract_v2(usdc_admin.clone()).address();
    let usdc = token::StellarAssetClient::new(&env, &usdc_id);
    usdc.mint(&merchant, &10_000_000_000); // 1000 USDC

    (env, merchant, oracle, usdc_id, usdc_admin)
}

fn deploy(env: &Env, merchant: &Address, oracle: &Address, usdc: &Address, bps: u32) -> Address {
    let id = env.register_contract(None, StelliatePayout);
    StelliatePayoutClient::new(env, &id).initialize(merchant, oracle, usdc, &bps);
    id
}

// ── Happy paths ───────────────────────────────────────────────────────────────

#[test]
fn test_deposit_and_balance() {
    let (env, merchant, oracle, usdc_id, _) = setup();
    let id = deploy(&env, &merchant, &oracle, &usdc_id, 1000);
    let client = StelliatePayoutClient::new(&env, &id);

    client.deposit(&1_000_000_000); // 100 USDC
    assert_eq!(client.balance(), 1_000_000_000);

    // merchant token balance should have decreased
    let usdc = token::Client::new(&env, &usdc_id);
    assert_eq!(usdc.balance(&merchant), 9_000_000_000);
}

#[test]
fn test_withdraw() {
    let (env, merchant, oracle, usdc_id, _) = setup();
    let id = deploy(&env, &merchant, &oracle, &usdc_id, 500);
    let client = StelliatePayoutClient::new(&env, &id);

    client.deposit(&1_000_000_000);
    client.withdraw(&400_000_000);
    assert_eq!(client.balance(), 600_000_000);
}

#[test]
fn test_pay_commission() {
    let (env, merchant, oracle, usdc_id, _) = setup();
    let id = deploy(&env, &merchant, &oracle, &usdc_id, 1000); // 10%
    let client = StelliatePayoutClient::new(&env, &id);
    let affiliate = Address::generate(&env);

    client.deposit(&1_000_000_000); // 100 USDC escrow
    client.register_affiliate(&affiliate);

    // sale_amount = 500 USDC → 10% = 50 USDC commission
    client.pay_commission(&affiliate, &5_000_000_000);

    let usdc = token::Client::new(&env, &usdc_id);
    assert_eq!(usdc.balance(&affiliate), 500_000_000);
    assert_eq!(client.balance(), 500_000_000);
}

// ── Error paths ───────────────────────────────────────────────────────────────

#[test]
#[should_panic]
fn test_double_initialize_panics() {
    let (env, merchant, oracle, usdc_id, _) = setup();
    let id = deploy(&env, &merchant, &oracle, &usdc_id, 1000);
    StelliatePayoutClient::new(&env, &id).initialize(&merchant, &oracle, &usdc_id, &500);
}

#[test]
#[should_panic]
fn test_zero_commission_bps_panics() {
    let (env, merchant, oracle, usdc_id, _) = setup();
    deploy(&env, &merchant, &oracle, &usdc_id, 0);
}

#[test]
#[should_panic]
fn test_deposit_zero_panics() {
    let (env, merchant, oracle, usdc_id, _) = setup();
    let id = deploy(&env, &merchant, &oracle, &usdc_id, 1000);
    StelliatePayoutClient::new(&env, &id).deposit(&0);
}

#[test]
#[should_panic]
fn test_withdraw_zero_panics() {
    let (env, merchant, oracle, usdc_id, _) = setup();
    let id = deploy(&env, &merchant, &oracle, &usdc_id, 1000);
    let client = StelliatePayoutClient::new(&env, &id);
    client.deposit(&1_000_000_000);
    client.withdraw(&0);
}

#[test]
#[should_panic]
fn test_withdraw_exceeds_balance_panics() {
    let (env, merchant, oracle, usdc_id, _) = setup();
    let id = deploy(&env, &merchant, &oracle, &usdc_id, 1000);
    let client = StelliatePayoutClient::new(&env, &id);
    client.deposit(&100_000_000);
    client.withdraw(&200_000_000);
}

#[test]
#[should_panic]
fn test_unregistered_affiliate_panics() {
    let (env, merchant, oracle, usdc_id, _) = setup();
    let id = deploy(&env, &merchant, &oracle, &usdc_id, 1000);
    let client = StelliatePayoutClient::new(&env, &id);
    let stranger = Address::generate(&env);
    client.deposit(&1_000_000_000);
    client.pay_commission(&stranger, &1_000_000_000);
}

#[test]
#[should_panic]
fn test_inactive_affiliate_panics() {
    let (env, merchant, oracle, usdc_id, _) = setup();
    let id = deploy(&env, &merchant, &oracle, &usdc_id, 1000);
    let client = StelliatePayoutClient::new(&env, &id);
    let affiliate = Address::generate(&env);

    client.deposit(&1_000_000_000);
    client.register_affiliate(&affiliate);
    client.deactivate_affiliate(&affiliate);
    client.pay_commission(&affiliate, &1_000_000_000);
}

#[test]
#[should_panic]
fn test_insufficient_escrow_panics() {
    let (env, merchant, oracle, usdc_id, _) = setup();
    let id = deploy(&env, &merchant, &oracle, &usdc_id, 1000);
    let client = StelliatePayoutClient::new(&env, &id);
    let affiliate = Address::generate(&env);

    client.deposit(&100_000_000); // 10 USDC
    client.register_affiliate(&affiliate);
    client.pay_commission(&affiliate, &100_000_000_000); // commission > escrow
}

#[test]
#[should_panic]
fn test_pay_commission_zero_sale_panics() {
    let (env, merchant, oracle, usdc_id, _) = setup();
    let id = deploy(&env, &merchant, &oracle, &usdc_id, 1000);
    let client = StelliatePayoutClient::new(&env, &id);
    let affiliate = Address::generate(&env);

    client.deposit(&1_000_000_000);
    client.register_affiliate(&affiliate);
    client.pay_commission(&affiliate, &0);
}
