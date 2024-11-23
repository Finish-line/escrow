use anchor_lang::prelude::*;

use crate::Escrow;
#[derive(Accounts)]
pub struct Take<'info> {
    #[account(mut)]
    taker: Signer<'info>,

    #[account(mut)]
    maker: SystemAccount<'info>,

    #[account(
        mut,
        close = taker,
        has_one = maker,
        seeds = [b"escrow", maker.key.as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
    )]
    escrow: Account<'info, Escrow>,

    system_program: Program<'info, System>,
}

impl<'info> Take<'info> {
    pub fn withdraw_and_close(&mut self) -> Result<()> {
        msg!("Withdrawing funds from escrow and closing it");

        let escrow_balance = self.escrow.to_account_info().lamports();
        **self.escrow.to_account_info().try_borrow_mut_lamports()? -= escrow_balance;
        **self.taker.to_account_info().try_borrow_mut_lamports()? += escrow_balance;

        self.escrow.close(self.taker.to_account_info())?;

        msg!("Escrow closed successfully");
        Ok(())
    }
}