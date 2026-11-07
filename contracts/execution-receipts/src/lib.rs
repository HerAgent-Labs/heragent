#![no_std]

mod errors;
mod storage;
mod types;

#[cfg(test)]
mod test;

use soroban_sdk::{contract, contractimpl, Address, BytesN, Env};

use errors::ContractError;
use storage::{
    get_admin, get_receipt as load_receipt, is_issuer_authorized, set_admin, set_issuer,
    set_receipt,
};
use types::Receipt;

#[contract]
pub struct ExecutionReceipts;

#[contractimpl]
impl ExecutionReceipts {
    /// Initialize the contract with an administrator.
    pub fn initialize(env: Env, admin: Address) -> Result<(), ContractError> {
        if get_admin(&env).is_some() {
            return Err(ContractError::AlreadyInitialized);
        }

        admin.require_auth();

        set_admin(&env, &admin);

        Ok(())
    }

    /// Authorize an account to register execution receipts.
    pub fn register_issuer(env: Env, issuer: Address) -> Result<(), ContractError> {
        let admin = get_admin(&env).ok_or(ContractError::NotInitialized)?;

        admin.require_auth();

        if is_issuer_authorized(&env, &issuer) {
            return Err(ContractError::IssuerAlreadyAuthorized);
        }

        set_issuer(&env, &issuer, true);

        Ok(())
    }

    /// Revoke an issuer's permission.
    pub fn revoke_issuer(env: Env, issuer: Address) -> Result<(), ContractError> {
        let admin = get_admin(&env).ok_or(ContractError::NotInitialized)?;

        admin.require_auth();

        if !is_issuer_authorized(&env, &issuer) {
            return Err(ContractError::IssuerNotAuthorized);
        }

        set_issuer(&env, &issuer, false);

        Ok(())
    }

    /// Register an execution receipt.
    pub fn register_receipt(
        env: Env,
        issuer: Address,
        receipt_id: BytesN<32>,
        result_hash: BytesN<32>,
        agent_ref_hash: BytesN<32>,
    ) -> Result<(), ContractError> {
        issuer.require_auth();

        if !is_issuer_authorized(&env, &issuer) {
            return Err(ContractError::IssuerNotAuthorized);
        }

        if load_receipt(&env, &receipt_id).is_some() {
            return Err(ContractError::ReceiptAlreadyExists);
        }

        let receipt = Receipt {
            receipt_id,
            result_hash,
            agent_ref_hash,
            issuer,
            recorded_at: env.ledger().timestamp(),
        };

        set_receipt(&env, &receipt);

        Ok(())
    }

    /// Retrieve a registered execution receipt.
    pub fn get_receipt(env: Env, receipt_id: BytesN<32>) -> Option<Receipt> {
        load_receipt(&env, &receipt_id)
    }

    /// Verify a result hash against a registered receipt.
    pub fn verify_receipt(env: Env, receipt_id: BytesN<32>, result_hash: BytesN<32>) -> bool {
        match load_receipt(&env, &receipt_id) {
            Some(receipt) => receipt.result_hash == result_hash,
            None => false,
        }
    }
}
