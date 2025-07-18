use anchor_lang::{prelude::*, system_program};

use anchor_spl:: {
    associated_token::{self, AssociatedToken}, token::{self, Token, TokenAccount, Mint}
};

use crate::{constant::constants::*, Pool, PoolConfig, POOL_CONFIG_SIZE, POOL_SIZE};

#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init, 
        seeds = [
            POOL_CONFIG_SEED,
            staking_token_mint.key().as_ref(), 
            underlaying_mint.key().as_ref()
        ],
        bump, 
        space = POOL_CONFIG_SIZE,
        payer = signer
    )]
    pub pool_config: Box<Account<'info, PoolConfig>>,

    #[account(
        init, 
        seeds = [
            POOL_SEED,
            staking_token_mint.key().as_ref(),
            underlaying_mint.key().as_ref(),
        ],
        bump,
        space = POOL_SIZE,
        payer = signer
    )]
    pub pool: Box<Account<'info, Pool>>,

    #[account(
        init,
        payer = signer,
        associated_token::mint = underlaying_mint,
        associated_token::authority = pool_config
    )]
    pub pool_underlaying_account: Box<Account<'info, TokenAccount>>,

    pub staking_token_mint: Box<Account<'info, Mint>>,

    pub underlaying_mint: Box<Account<'info, Mint>>,

    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,

    #[account(address = associated_token::ID)]
    pub associated_token_program: Program<'info, AssociatedToken>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
    
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> InitializePool<'info> {
    pub fn process(&mut self, exchange_rate: f64, unbound_delay: i64) -> Result<()> {
        let pool_config = &mut self.pool_config;
        let pool = &mut self.pool;

        pool_config.owner = self.signer.key();
        pool_config.staking_token_mint = self.staking_token_mint.key();
        pool_config.underlaying_mint = self.underlaying_mint.key();
        pool_config.unbound_delay = unbound_delay;

        pool.exchange_rate = 1.0;
        pool.last_updated = Clock::get()?.unix_timestamp;
        
        Ok(())
    }
}