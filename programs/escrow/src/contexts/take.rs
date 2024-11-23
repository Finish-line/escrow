use anchor_lang::prelude::*;

use crate::Escrow;
#[derive(Accounts)]
pub struct Take<'info> {
    /// The taker who will withdraw funds
    #[account(mut)]
    taker: Signer<'info>,

    /// The maker who created the escrow
    #[account(mut)]
    maker: SystemAccount<'info>,

    /// The escrow account storing the SOL and metadata
    #[account(
        mut,
        close = taker,
        has_one = maker,
        seeds = [b"escrow", maker.key.as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
    )]
    escrow: Account<'info, Escrow>,

    /// The system program for transferring SOL
    system_program: Program<'info, System>,
}

impl<'info> Take<'info> {
    /// Withdraw all SOL from the escrow and close it
    pub fn withdraw_and_close(&mut self) -> Result<()> {
        msg!("Withdrawing funds from escrow and closing it");

        // Transfer all SOL from escrow to taker
        let escrow_balance = self.escrow.to_account_info().lamports();
        **self.escrow.to_account_info().try_borrow_mut_lamports()? -= escrow_balance;
        **self.taker.to_account_info().try_borrow_mut_lamports()? += escrow_balance;

        // Close escrow account by transferring rent exemption to the taker
        self.escrow.close(self.taker.to_account_info())?;

        msg!("Escrow closed successfully");
        Ok(())
    }
}