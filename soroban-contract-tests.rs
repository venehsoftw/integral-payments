#![cfg(test)]
use super::*;
use soroban_sdk::{
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation, MockAuth, MockAuthInvoke},
    Address, Env, String, Vec,
};

// Test contract initialization
#[test]
fn test_initialize_contract() {
    let env = Env::default();
    let contract_id = env.register_contract(None, PaymentContract);
    let client = PaymentContractClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    let default_fee = 250u32; // 2.5%

    env.mock_all_auths();
    
    let result = client.initialize(&owner, &default_fee);
    assert!(result.is_ok());
}

#[test]
fn test_initialize_with_invalid_fee() {
    let env = Env::default();
    let contract_id = env.register_contract(None, PaymentContract);
    let client = PaymentContractClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    let invalid_fee = 10001u32; // > 100%

    env.mock_all_auths();
    
    let result = client.try_initialize(&owner, &invalid_fee);
    assert!(result.is_err());
}

#[test]
fn test_register_business() {
    let env = Env::default();
    let contract_id = env.register_contract(None, PaymentContract);
    let client = PaymentContractClient::new(&env, &contract_id);

    // Initialize contract
    let owner = Address::generate(&env);
    let default_fee = 250u32;
    env.mock_all_auths();
    client.initialize(&owner, &default_fee).unwrap();

    // Register business
    let business_name = String::from_str(&env, "Test Store");
    let business_owner = Address::generate(&env);
    let fee_recipient = Address::generate(&env);
    let fee_percentage = 300u32; // 3%

    let result = client.register_business(
        &business_name,
        &business_owner,
        &fee_recipient,
        &fee_percentage,
    );
    assert!(result.is_ok());

    // Verify business was registered
    let business_config = client.get_business_config(&business_name).unwrap();
    assert_eq!(business_config.name, business_name);
    assert_eq!(business_config.owner, business_owner);
    assert_eq!(business_config.fee_recipient, fee_recipient);
    assert_eq!(business_config.default_fee_percentage, fee_percentage);
    assert!(business_config.is_active);
}

#[test]
fn test_create_payment_request() {
    let env = Env::default();
    let contract_id = env.register_contract(None, PaymentContract);
    let client = PaymentContractClient::new(&env, &contract_id);

    // Initialize contract
    let owner = Address::generate(&env);
    let default_fee = 250u32;
    env.mock_all_auths();
    client.initialize(&owner, &default_fee).unwrap();

    // Register business
    let business_name = String::from_str(&env, "Test Store");
    let business_owner = Address::generate(&env);
    let fee_recipient = Address::generate(&env);
    let fee_percentage = 300u32;

    client.register_business(
        &business_name,
        &business_owner,
        &fee_recipient,
        &fee_percentage,
    ).unwrap();

    // Create payment request
    let amount = 1000000i128; // 100 XLM (in stroops)
    let description = String::from_str(&env, "Test payment");
    let denomination = String::from_str(&env, "XLM");
    let requester = Address::generate(&env);
    
    let mut authorized_addresses = Vec::new(&env);
    authorized_addresses.push_back(Address::generate(&env));
    authorized_addresses.push_back(Address::generate(&env));
    authorized_addresses.push_back(Address::generate(&env));

    let payment_id = client.create_payment_request(
        &amount,
        &business_name,
        &description,
        &denomination,
        &authorized_addresses,
        &requester,
        &None,
    ).unwrap();

    // Verify payment request
    let payment_request = client.get_payment_request(&payment_id).unwrap();
    assert_eq!(payment_request.amount, amount);
    assert_eq!(payment_request.business_name, business_name);
    assert_eq!(payment_request.description, description);
    assert_eq!(payment_request.denomination, denomination);
    assert_eq!(payment_request.requester, requester);
    assert_eq!(payment_request.authorized_addresses, authorized_addresses);
    assert_eq!(payment_request.fee_percentage, fee_percentage);
    
    match payment_request.status {
        PaymentStatus::Pending => {},
        _ => panic!("Payment should be pending"),
    }
}

#[test]
fn test_create_payment_request_with_custom_fee() {
    let env = Env::default();
    let contract_id = env.register_contract(None, PaymentContract);
    let client = PaymentContractClient::new(&env, &contract_id);

    // Initialize and setup business
    let owner = Address::generate(&env);
    let default_fee = 250u32;
    env.mock_all_auths();
    client.initialize(&owner, &default_fee).unwrap();

    let business_name = String::from_str(&env, "Test Store");
    let business_owner = Address::generate(&env);
    let fee_recipient = Address::generate(&env);
    let fee_percentage = 300u32;

    client.register_business(
        &business_name,
        &business_owner,
        &fee_recipient,
        &fee_percentage,
    ).unwrap();

    // Create payment request with custom fee
    let amount = 1000000i128;
    let description = String::from_str(&env, "Test payment");
    let denomination = String::from_str(&env, "XLM");
    let requester = Address::generate(&env);
    let custom_fee = 500u32; // 5%
    
    let mut authorized_addresses