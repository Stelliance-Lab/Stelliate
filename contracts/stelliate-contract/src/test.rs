#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, token, Address, Env};

fn setup() -> (Env, Address, Address, Address, Address) {
    let env = Env::default();
    env.mock_all_auths();

    let merchant = Address::generate(&env);
    let affiliate = Address::generate(&env);

    // Deploy a mock USDC token
    let usdc_admin = Address::generate(&env);
    let usdc_id = env.register_stellar_asset_contract_v2(usdc_admin.clone()).address();
    let usdc = token::StellarAssetClient::new(&env, &usdc_id);

    // Fund merchant with 1000 USDC (7 decimals = 10_000_000_000 stroops)
    usdc.mint(&merchant, &10_000_000_000);

    (env, merchant, affiliate, usdc_id, usdc_admin)
}

fn deploy(env: &Env, merchant: &Address, usdc: &Address, bps: u32) -> Address {
    let contract_id = env.register_contract(None, StelliatePayout);
    let client = StelliatePayoutClient::new(env, &contract_id);
    client.initialize(merchant, usdc, &bps);
    contract_id
}

#[test]
fn test_initialize_and_deposit() {
    let (env, merchant, _, usdc_id, _) = setup();
    let contract_id = deploy(&env, &merchant, &usdc_id, 1000); // 10%
    let client = StelliatePayoutClient::new(&env, &contract_id);

    client.deposit(&500_000_000); // 50 USDC
    assert_eq!(client.balance(), 500_000_000);
}

#[test]
fn test_pay_commission() {
    let (env, merchant, affiliate, usdc_id, _) = setup();
    let contract_id = deploy(&env, &merchant, &usdc_id, 1000); // 10%
    let client = StelliatePayoutClient::new(&env, &contract_id);

    client.deposit(&1_000_000_000); // 100 USDC
    client.register_affiliate(&affiliate);

    // Sale of 500 USDC → 10% = 50 USDC commission
    client.pay_commission(&affiliate, &5_000_000_000);

    let usdc_client = token::Client::new(&env, &usdc_id);
    assert_eq!(usdc_client.balance(&affiliate), 500_000_000); // 50 USDC
    assert_eq!(client.balance(), 500_000_000); // 50 USDC remaining
}

#[test]
fn test_withdraw() {
    let (env, merchant, _, usdc_id, _) = setup();
    let contract_id = deploy(&env, &merchant, &usdc_id, 500);
    let client = StelliatePayoutClient::new(&env, &contract_id);

    client.deposit(&1_000_000_000);
    client.withdraw(&400_000_000);
    assert_eq!(client.balance(), 600_000_000);
}

#[test]
#[should_panic]
fn test_insufficient_escrow_panics() {
    let (env, merchant, affiliate, usdc_id, _) = setup();
    let contract_id = deploy(&env, &merchant, &usdc_id, 1000);
    let client = StelliatePayoutClient::new(&env, &contract_id);

    client.deposit(&100_000_000); // 10 USDC
    client.register_affiliate(&affiliate);
    // Sale of 10000 USDC → commission 1000 USDC, but only 10 in escrow
    client.pay_commission(&affiliate, &100_000_000_000);
}

#[test]
#[should_panic]
fn test_inactive_affiliate_panics() {
    let (env, merchant, affiliate, usdc_id, _) = setup();
    let contract_id = deploy(&env, &merchant, &usdc_id, 1000);
    let client = StelliatePayoutClient::new(&env, &contract_id);

    client.deposit(&1_000_000_000);
    client.register_affiliate(&affiliate);
    client.deactivate_affiliate(&affiliate);
    client.pay_commission(&affiliate, &1_000_000_000);
}

#[test]
#[should_panic]
fn test_double_initialize_panics() {
    let (env, merchant, _, usdc_id, _) = setup();
    let contract_id = deploy(&env, &merchant, &usdc_id, 1000);
    let client = StelliatePayoutClient::new(&env, &contract_id);
    // Second initialize should fail
    client.initialize(&merchant, &usdc_id, &500);
}
