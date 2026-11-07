use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum ContractError {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    Unauthorized = 3,
    IssuerNotAuthorized = 4,
    IssuerAlreadyAuthorized = 5,
    ReceiptAlreadyExists = 6,
    ReceiptNotFound = 7,
}
