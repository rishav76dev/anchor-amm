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

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

