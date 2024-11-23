use anchor_lang::prelude::*;

mod contexts;
use contexts::*;

mod state;
use state::*;

// TODO: Update with your actual program ID
declare_id!("3V5tgL8vxgC3jKiyXxAzmdkkCi8sTs1TMwfvLiNpBMXx");

#[program]
pub mod escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, amount: u64) -> Result<()> {
        ctx.accounts.save_escrow(
            seed,
            ctx.bumps.escrow
        )?;
        ctx.accounts.deposit(amount)
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        // Withdraw SOL to the maker
        ctx.accounts.withdraw_and_close()
    }

    // Uncomment if implementing a refund function later
    // /// Refunds SOL back to the maker and closes the escrow account
    // pub fn refund(ctx: Context<Refund>) -> Result<()> {
    //     ctx.accounts.withdraw_and_close()
    // }
}

#[error_code]
pub enum ErrorCode {
    #[msg("The required PDA bump was not provided.")]
    MissingBump,
}