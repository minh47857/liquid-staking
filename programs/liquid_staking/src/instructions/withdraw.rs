use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

use crate::error::ErrorCode;
use crate::{constant::constants::{POOL_CONFIG_SEED, USER_UNBOUND_REQUEST_SEED}, transfer_token_with_signer, PoolConfig, UserUnboundRequest};


#[derive(Accounts)]
pub struct WithDraw<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    pub staking_token_mint: Account<'info, Mint>,

    pub underlaying_mint: Account<'info, Mint>,

    #[account(
        mut, 
        seeds = [
            POOL_CONFIG_SEED,
            staking_token_mint.key().as_ref(),
            underlaying_mint.key().as_ref(),
        ],
        bump,
    )]
    pub pool: Box<Account<'info, PoolConfig>>,

    #[account(
        mut,
        associated_token::mint = underlaying_mint,
        associated_token::authority = signer,
    )]
    pub user_underlaying_account: Box<Account<'info, TokenAccount>>,

    #[account(
        mut, 
        associated_token::mint = underlaying_mint,
        associated_token::authority = pool,
    )]
    pub pool_underlaying_account: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [
            USER_UNBOUND_REQUEST_SEED,
            signer.key().as_ref(),
            pool.key().as_ref(),
        ],
        bump,
    )]
    pub user_unbound_request: Box<Account<'info, UserUnboundRequest>>,

    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,

    pub clock: Sysvar<'info, Clock>,
}

impl<'info> WithDraw<'info> {
    pub fn process(&mut self) -> Result<()> {
        let pool = &self.pool;
        let user_unbound_request = &mut self.user_unbound_request;

        let current_time = self.clock.unix_timestamp;
        let amount = user_unbound_request.unstaked_amount;

        require!(current_time >= user_unbound_request.withdraw_timestamp, ErrorCode::UnboundDelayNotPassed);
        
        transfer_token_with_signer(
            self.pool_underlaying_account.to_account_info(),
            self.pool.to_account_info(),
            self.user_underlaying_account.to_account_info(),
            &[&pool.auth_seeds()], 
            &self.token_program, 
            amount,
        )?;

        user_unbound_request.is_unstaked = false;
        user_unbound_request.unstaked_amount = 0;
        user_unbound_request.withdraw_timestamp = 0;

        Ok(())
    }
}
