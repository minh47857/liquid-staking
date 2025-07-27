use anchor_lang::prelude::*;

use crate::constant::constants::POOL_CONFIG_SEED;

#[account]
pub struct PoolConfig {
    pub owner: Pubkey,
    pub underlaying_mint: Pubkey,
    pub staking_token_mint: Pubkey,
    pub unbound_delay: i64,
    pub bump: [u8; 1],
}

pub const POOL_CONFIG_SIZE: usize = 8 + 32 + 32 + 32 + 8 + 1;

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
    pub total_staked: u64,
    pub accumulated_reward: u64,
    pub acoount_admin: [Pubkey; 128],
}

impl<'info> Pool {
    pub fn update_exchange_rate(&mut self, total_lst: u64) -> Result<()> {
        if total_lst != 0 {
            self.exchange_rate =
                (self.total_staked + self.accumulated_reward) as f64 / total_lst as f64;
        } else {
            self.exchange_rate = 1.0;
        }
        Ok(())
    }
}

pub const POOL_SIZE: usize = 8 + 8 + 8 + 8 + 8;
