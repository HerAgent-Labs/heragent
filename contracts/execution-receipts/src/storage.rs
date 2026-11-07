use soroban_sdk::{Address, BytesN, Env};

use crate::types::{DataKey, Receipt};

pub fn get_admin(env: &Env) -> Option<Address> {
    env.storage().instance().get(&DataKey::Admin)
}

pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&DataKey::Admin, admin);
}

pub fn is_issuer_authorized(env: &Env, issuer: &Address) -> bool {
    env.storage()
        .persistent()
        .get(&DataKey::Issuer(issuer.clone()))
        .unwrap_or(false)
}

pub fn set_issuer(env: &Env, issuer: &Address, authorized: bool) {
    env.storage()
        .persistent()
        .set(&DataKey::Issuer(issuer.clone()), &authorized);
}

pub fn get_receipt(env: &Env, receipt_id: &BytesN<32>) -> Option<Receipt> {
    env.storage()
        .persistent()
        .get(&DataKey::Receipt(receipt_id.clone()))
}

pub fn set_receipt(env: &Env, receipt: &Receipt) {
    env.storage()
        .persistent()
        .set(&DataKey::Receipt(receipt.receipt_id.clone()), receipt);
}
