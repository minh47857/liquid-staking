use anchor_lang::prelude::*;

pub mod state;
pub use state::*;
pub mod instructions;
pub use instructions::*;
pub mod constant;
pub mod utils;
pub use utils::*;
pub mod error;

declare_id!("BhTX9xkg1JVAqQCNwqpP9f7uH59KsmRPNDQ31BQBqacv");

#[program]
pub mod liquid_staking {
    use super::*;

    pub fn initialize(
        ctx: Context<InitializePool>,
        exchange_rate: f64,
        unbound_delay: i64,
    ) -> Result<()> {
        ctx.accounts.process(exchange_rate, unbound_delay)
    }

    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        ctx.accounts.process(amount)
    }

    pub fn unstake(ctx: Context<UnStake>, amount: u64) -> Result<()> {
        ctx.accounts.process(amount)
    }

    pub fn withdraw(ctx: Context<WithDraw>) -> Result<()> {
        ctx.accounts.process()
    }
}
