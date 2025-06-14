#![allow(unexpected_cfgs)]
pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;
pub use instructions::*;

declare_id!("Bh7sunHvQn67bepQJXXLrAciBKWP9Cn19WsSW2PRhyFg");

#[program]
pub mod anchor_escrow {

    use super::*;

    pub fn make(ctx: Context<MakeOffer>, seed:u64, receive: u64, deposit: u64) -> Result<()> {
        ctx.accounts.init_escrow(seed, receive, ctx.bumps)?;
        ctx.accounts.deposit(deposit)?;
       
        Ok(())
    }

    pub fn exchange(ctx: Context<TakeOffer>) -> Result<()> {
        ctx.accounts.send_wanted_tokens_to_maker()?;
        ctx.accounts.withdraw_and_close_vault()?;
        Ok(())
    }

    pub fn refund(ctx: Context<RefundOffer>) -> Result<()> {
        ctx.accounts.withdraw_and_close_vault()?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
