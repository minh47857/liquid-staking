use anchor_lang::prelude::*;

#[account]
pub struct UserUnboundRequest {
    pub owner: Pubkey,
    pub pool_config: Pubkey,
    pub amount: u64,
    pub withdraw_timestamp: i64,
    pub is_unstaked: bool,
}

pub const USER_UNBOUND_REQUEST_SIZE: usize = 8 + 32 + 32 + 8 + 8 + 1 + 1;
