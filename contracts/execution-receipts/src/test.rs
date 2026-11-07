#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, BytesN, Env};

#[test]
fn test_initialize_and_register_receipt() {
    let env = Env::default();

    // Mock authorization for contract testing.
    env.mock_all_auths();

    // Register the contract.
    let contract_id = env.register(ExecutionReceipts, ());
    let client = ExecutionReceiptsClient::new(&env, &contract_id);

    // Generate test identities.
    let admin = Address::generate(&env);
    let issuer = Address::generate(&env);

    // Initialize the contract.
    client.initialize(&admin);

    // Authorize an issuer.
    client.register_issuer(&issuer);

    // Prepare receipt hashes.
    let receipt_id = BytesN::from_array(&env, &[1u8; 32]);
    let result_hash = BytesN::from_array(&env, &[2u8; 32]);
    let agent_ref_hash = BytesN::from_array(&env, &[3u8; 32]);

    // Register an execution receipt.
    client.register_receipt(&issuer, &receipt_id, &result_hash, &agent_ref_hash);

    // Retrieve the stored receipt.
    let receipt = client
        .get_receipt(&receipt_id)
        .expect("Receipt should exist");

    // Verify receipt contents.
    assert_eq!(receipt.receipt_id, receipt_id);
    assert_eq!(receipt.result_hash, result_hash);
    assert_eq!(receipt.agent_ref_hash, agent_ref_hash);
    assert_eq!(receipt.issuer, issuer);

    // Verify the result hash.
    assert!(client.verify_receipt(&receipt_id, &result_hash));

    // Verify that an incorrect result hash fails verification.
    let incorrect_hash = BytesN::from_array(&env, &[9u8; 32]);

    assert!(!client.verify_receipt(&receipt_id, &incorrect_hash));
}

#[test]
fn test_receipt_not_found() {
    let env = Env::default();

    let contract_id = env.register(ExecutionReceipts, ());
    let client = ExecutionReceiptsClient::new(&env, &contract_id);

    let receipt_id = BytesN::from_array(&env, &[5u8; 32]);

    assert!(client.get_receipt(&receipt_id).is_none());
    assert!(!client.verify_receipt(&receipt_id, &BytesN::from_array(&env, &[6u8; 32])));
}
