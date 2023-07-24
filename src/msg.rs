use cosmwasm_std::{Binary, Uint128};
use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub enum ResponseStatus {
    Success,
    Failure,
}

#[cw_serde]
pub enum ContractStatus {
    Active,
    Stopped,
}

#[cw_serde]
pub struct NftToken {
    pub token_id: String,
    pub viewing_key: String,
}

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: Option<String>,
    pub lock_periods: Vec<u64>,
    pub tier_contract: String,
    pub tier_contract_hash: String,
    pub nft_contract: String,
    pub nft_contract_hash: String,
}

#[cw_serde]
pub enum PaymentMethod {
    Native,
    Token {
        contract: String,
        code_hash: String,
    },
}

#[cw_serde]
pub enum Whitelist {
    Empty {
        with: Option<Vec<String>>,
    },
    Shared {
        with_blocked: Option<Vec<String>>,
    },
}

#[cw_serde]
pub enum ExecuteMsg {
    ChangeAdmin {
        admin: String,
        padding: Option<String>,
    },
    ChangeStatus {
        status: ContractStatus,
        padding: Option<String>,
    },
    StartIdo {
        start_time: u64,
        end_time: u64,
        token_contract: String,
        token_contract_hash: String,
        price: Uint128,
        soft_cap: Uint128,
        payment: PaymentMethod,
        total_amount: Uint128,
        tokens_per_tier: Vec<Uint128>,
        padding: Option<String>,
        whitelist: Whitelist,
    },
    WhitelistAdd {
        addresses: Vec<String>,
        ido_id: u32,
        padding: Option<String>,
    },
    WhitelistRemove {
        addresses: Vec<String>,
        ido_id: u32,
        padding: Option<String>,
    },
    BuyTokens {
        ido_id: u32,
        amount: Uint128,
        viewing_key: Option<String>,
        padding: Option<String>,
    },
    RecvTokens {
        ido_id: u32,
        start: Option<u32>,
        limit: Option<u32>,
        purchase_indices: Option<Vec<u32>>,
        padding: Option<String>,
    },
    Withdraw {
        ido_id: u32,
        padding: Option<String>,
    },
}

#[cw_serde]
pub enum HandleAnswer {
    ChangeAdmin {
        status: ResponseStatus,
    },
    ChangeStatus {
        status: ResponseStatus,
    },
    StartIdo {
        ido_id: u32,
        status: ResponseStatus,
    },
    WhitelistAdd {
        status: ResponseStatus,
    },
    WhitelistRemove {
        status: ResponseStatus,
    },
    BuyTokens {
        amount: Uint128,
        unlock_time: u64,
        status: ResponseStatus,
    },
    RecvTokens {
        amount: Uint128,
        status: ResponseStatus,
        ido_success: bool,
    },
    Withdraw {
        ido_amount: Uint128,
        payment_amount: Uint128,
        status: ResponseStatus,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Binary)]
    Config {},
    #[returns(Binary)]
    IdoAmount {},
    #[returns(Binary)]
    IdoInfo {
        ido_id: u32,
    },
    #[returns(Binary)]
    InWhitelist {
        address: String,
        ido_id: u32,
    },
    #[returns(Binary)]
    IdoListOwnedBy {
        address: String,
        start: u32,
        limit: u32,
    },
    #[returns(Binary)]
    Purchases {
        ido_id: u32,
        address: String,
        start: Option<u32>,
        limit: Option<u32>,
    },
    #[returns(Binary)]
    ArchivedPurchases {
        ido_id: u32,
        address: String,
        start: u32,
        limit: u32,
    },
    #[returns(Binary)]
    UserInfo {
        address: String,
        ido_id: Option<u32>,
    },
    #[returns(Binary)]
    TierInfo {
        address: String,
        viewing_key: Option<String>,
    },
}

#[cw_serde]
pub struct PurchaseAnswer {
    pub tokens_amount: Uint128,
    pub timestamp: u64,
    pub unlock_time: u64,
}

#[cw_serde]
pub enum QueryAnswer {
    Config {
        admin: String,
        tier_contract: String,
        tier_contract_hash: String,
        nft_contract: String,
        nft_contract_hash: String,
        lock_periods: Vec<u64>,
    },
    IdoAmount {
        amount: u32,
    },
    IdoInfo {
        admin: String,
        start_time: u64,
        end_time: u64,
        token_contract: String,
        token_contract_hash: String,
        price: Uint128,
        participants: u64,
        payment: PaymentMethod,
        sold_amount: Uint128,
        total_tokens_amount: Uint128,
        total_payment: Uint128,
        soft_cap: Uint128,
        withdrawn: bool,
        shared_whitelist: bool,
        remaining_per_tiers: Vec<Uint128>,
    },
    InWhitelist {
        in_whitelist: bool,
    },
    IdoListOwnedBy {
        ido_ids: Vec<u32>,
        amount: u32,
    },
    Purchases {
        purchases: Vec<PurchaseAnswer>,
        amount: u32,
    },
    ArchivedPurchases {
        purchases: Vec<PurchaseAnswer>,
        amount: u32,
    },
    UserInfo {
        total_payment: Uint128,
        total_tokens_bought: Uint128,
        total_tokens_received: Uint128,
    },
    TierInfo {
        tier: u8,
        nft_tier: u8,
    },
}
