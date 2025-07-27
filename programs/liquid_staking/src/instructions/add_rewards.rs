use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

use crate::{constant::constants::*, transfer_token_user, Pool, PoolConfig};

#[derive(Accounts)]
pub struct AddReward<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

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

    #[account(
        mut,
        associated_token::mint = underlaying_mint,
        associated_token::authority = admin,
    )]
    pub user_underlaying_account: Box<Account<'info, TokenAccount>>,

    #[account(
        mut, 
        associated_token::mint = underlaying_mint,
        associated_token::authority = pool_config,
    )]
    pub pool_underlaying_account: Box<Account<'info, TokenAccount>>,

    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,
}

impl<'info> AddReward<'info> {
    pub fn process(&mut self, reward: u64) -> Result<()> {
        let pool = &mut self.pool;

        pool.accumulated_reward += reward;

        transfer_token_user(
            self.user_underlaying_account.to_account_info(), 
            &self.admin, 
            self.pool_underlaying_account.to_account_info(), 
            &self.token_program, 
            reward,
        )?;

        Ok(())
    }
}