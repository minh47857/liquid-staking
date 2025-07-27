use std::ops::{Div};

use anchor_lang::{prelude::*, system_program};

use anchor_spl::{
    associated_token::{self, AssociatedToken},
    token::{self, Mint, Token, TokenAccount},
};

use crate::{constant::constants::*,POOL_CONFIG_SIZE, POOL_SIZE, mint_token, transfer_token_user, Pool, PoolConfig, UserUnboundRequest, USER_UNBOUND_REQUEST_SIZE};

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init_if_needed, 
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
        init_if_needed, 
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

    #[account(mut)]
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
}

impl<'info> Stake<'info> {
    pub fn process(&mut self, amount: u64) -> Result<()> {
        let pool = &mut self.pool;
        let pool_config = &self.pool_config;
        let user_unbound_request = &mut self.user_unbound_request;

        pool.update_exchange_rate(self.staking_token_mint.supply)?;
        pool.total_staked += amount;

        let staking_token_amount = (amount as f64).div(pool.exchange_rate).floor() as u64;

        transfer_token_user(
            self.user_underlaying_account.to_account_info(),
            &self.signer,
            self.pool_underlaying_account.to_account_info(), 
            &self.token_program, 
            amount
        )?;

        msg!("hello");

        mint_token(
            self.staking_token_mint.to_account_info(), 
            self.pool_config.to_account_info(), 
            self.user_staking_token_account.to_account_info(), 
            &[&pool_config.auth_seeds()[..]],
            &self.token_program, 
            staking_token_amount,
        )?;        

        if user_unbound_request.owner == Pubkey::default() {
            user_unbound_request.owner = self.signer.key();
            user_unbound_request.pool_config = self.pool_config.key();
            user_unbound_request.amount = 0;
            user_unbound_request.withdraw_timestamp = 0;
            user_unbound_request.is_unstaked = false;
        };

        Ok(())
    
    }
}