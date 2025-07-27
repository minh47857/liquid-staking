use std::ops::Mul;

use anchor_lang::{prelude::*, system_program};

use anchor_spl::{
    associated_token::{self, AssociatedToken},
    token::{self, Mint, Token, TokenAccount},
};


use crate::{error::ErrorCode, Pool};
use crate::{burn_token, constant::constants::*, PoolConfig, UserUnboundRequest, USER_UNBOUND_REQUEST_SIZE};

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [
            POOL_CONFIG_SEED,
            staking_token_mint.key().as_ref(),
            underlaying_mint.key().as_ref(),
        ],
        bump,
    )]
    pub pool_config: Box<Account<'info, PoolConfig>>,

    #[account(
        mut, 
        seeds = [
            POOL_SEED,
            staking_token_mint.key().as_ref(),
            underlaying_mint.key().as_ref(),
        ],
        bump,
    )]
    pub pool: Box<Account<'info, Pool>>,

    pub underlaying_mint: Box<Account<'info, Mint>>,

    #[account(mut)]
    pub staking_token_mint: Box<Account<'info, Mint>>,

    #[account(
        mut, 
        associated_token::mint = underlaying_mint,
        associated_token::authority = pool_config,
    )]
    pub pool_underlaying_account: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = underlaying_mint,
        associated_token::authority = signer,
    )]
    pub user_underlaying_account: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = staking_token_mint,
        associated_token::authority = signer,
    )]
    pub user_staking_token_account: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = signer,
        seeds = [
            USER_UNBOUND_REQUEST_SEED,
            signer.key().as_ref(),
            pool.key().as_ref(),
        ],
        bump,
        space = USER_UNBOUND_REQUEST_SIZE,
    )]
    pub user_unbound_request: Box<Account<'info, UserUnboundRequest>>,

    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,

    #[account(address = associated_token::ID)]
    pub associated_token_program: Program<'info, AssociatedToken>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    pub clock: Sysvar<'info, Clock>,
}

impl<'info> Unstake<'info> {
    pub fn process(&mut self, amount: u64) -> Result<()> {
        let pool = &mut self.pool;
        let pool_config = &self.pool_config;
        let user_unbound_request = &mut self.user_unbound_request;
        
        require!(!user_unbound_request.is_unstaked, ErrorCode::UserAlreadyUnstaked);
        
        pool.update_exchange_rate(self.staking_token_mint.supply)?;
        
        let underlaying_amount = (amount as f64).mul(pool.exchange_rate).round() as u64;
        msg!("=== UNSTAKE CALCULATION ===");
        msg!("Amount to unstake: {}", amount);
        msg!("Pool exchange rate: {}", pool.exchange_rate);
        msg!("Underlying amount: {}", underlaying_amount);
        pool.total_staked -= underlaying_amount;

        burn_token(
            &self.signer,
            self.staking_token_mint.to_account_info(),
            self.user_staking_token_account.to_account_info(),
            &self.token_program,
            amount,
        )?;

        user_unbound_request.is_unstaked = true;
        user_unbound_request.amount = underlaying_amount;
        user_unbound_request.withdraw_timestamp = self.clock.unix_timestamp + pool_config.unbound_delay;

        Ok(())
    }
}