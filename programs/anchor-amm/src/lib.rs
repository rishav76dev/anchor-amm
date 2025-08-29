use anchor_lang::prelude::*;

pub mod error;
pub mod constants;
pub mod state;
pub mod instructions;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("HuMUFVKhxNCEoPKJC5RNySYVLde3foDVTtdHBLVeQzdy");

#[program]
pub mod anchor_amm {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>, seed: u64, fee: u16, authority: Option<Pubkey>) -> Result<()> {
        ctx.accounts.initialize(seed, fee, authority, &ctx.bumps)?;

        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64, max_x: u64, max_y: u64) -> Result<()> {
        ctx.accounts.deposit(amount, max_x, max_y)?;
        Ok(())
    }

    pub fn swap(ctx: Context<Swap>, is_x: bool, amount: u64, min: u64) -> Result<()> {
        ctx.accounts.swap(is_x, amount, min)?;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64, min_x: u64, min_y: u64) -> Result<()>{
        ctx.accounts.withdraw(amount, min_x, min_y)?;

        Ok(())
    }

    pub fn lock(ctx: Context<Update>) -> Result<()>{
        ctx.accounts.lock()?;
        Ok(())
    }

    pub fn unlock(ctx: Context<Update>) -> Result<()>{
        ctx.accounts.unlock()?;
        Ok(())
    }
}

