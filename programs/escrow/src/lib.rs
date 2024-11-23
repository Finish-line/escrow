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

    /// Creates an escrow account and deposits SOL into it
    pub fn make(ctx: Context<Make>, seed: u64, amount: u64) -> Result<()> {
        // Save escrow metadata in the account
        ctx.accounts.save_escrow(seed, ctx.bumps.get("escrow").ok_or(ErrorCode::MissingBump)?)?;
        
        // Deposit the specified amount of SOL into the escrow account
        ctx.accounts.deposit(amount)
    }

    /// Allows the taker to withdraw SOL from the escrow and closes the escrow account
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