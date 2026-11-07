use soroban_sdk::{contracttype, Address, BytesN};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Receipt {
    pub receipt_id: BytesN<32>,
    pub result_hash: BytesN<32>,
    pub agent_ref_hash: BytesN<32>,
    pub issuer: Address,
    pub recorded_at: u64,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Issuer(Address),
    Receipt(BytesN<32>),
}
