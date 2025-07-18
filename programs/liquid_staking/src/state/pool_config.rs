use anchor_lang::prelude::*;

use crate::constant::constants::POOL_CONFIG_SEED;

#[account]
pub struct PoolConfig {
    pub owner: Pubkey,
    pub underlaying_mint: Pubkey,
    pub staking_token_mint: Pubkey,
    pub total_staked: u64,
    pub unbound_delay: i64,
    pub rps: u64,
    pub bump: [u8; 1],
}

pub const POOL_CONFIG_SIZE: usize = 8 + 32 + 32 + 32 + 8 + 8 + 8 + 1;

impl PoolConfig {
    pub fn auth_seeds<'a>(&'a self) -> [&'a [u8]; 4] {
        [
            POOL_CONFIG_SEED,
            self.staking_token_mint.as_ref(),
            self.underlaying_mint.as_ref(),
            self.bump.as_ref(),
        ]
    }
}

#[account]
pub struct Pool {
    pub exchange_rate: f64,
    pub last_updated: i64,
}

impl<'info> Pool {
    pub fn update_exchange_rate(&mut self, pool_config: &Account<'info, PoolConfig>) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp;
        let pass_time = current_time - self.last_updated;
        self.last_updated = current_time;
        self.exchange_rate += (pass_time as f64) * (pool_config.rps as f64);
        Ok(())
    }
}

pub const POOL_SIZE: usize = 8 + 8 + 8;
