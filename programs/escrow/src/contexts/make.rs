use anchor_lang::prelude::*;


use crate::Escrow;


impl Escrow {
    pub const INIT_SPACE: usize = 8 + 32 + 8 + 1; // seed (8) + maker (32) + receive (8) + bump (1)
}


#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Make<'info> {
    #[account(mut)]
    maker: Signer<'info>,

    /// Escrow account for holding SOL
    #[account(
        init,
        payer = maker,
        space = 8 + Escrow::INIT_SPACE,
        seeds = [b"escrow", maker.key.as_ref(), seed.to_le_bytes().as_ref()],
        bump,
    )]
    escrow: Account<'info, Escrow>,

    system_program: Program<'info, System>,
}

impl<'info> Make<'info> {
    /// Initialize and save the escrow account with the SOL deposit
    pub fn save_escrow(&mut self, seed: u64, bump: u8) -> Result<()> {
        self.escrow.set_inner(Escrow {
            seed,
            maker: self.maker.key(),
            balance: 0, // Initial balance is 0; will be updated upon deposit
            bump,
        });
        Ok(())
    }

    /// Deposit SOL from the maker into the escrow account
    pub fn deposit(&mut self, lamports: u64) -> Result<()> {
        let escrow_account = &mut self.escrow;

        // Transfer SOL from maker to the escrow PDA
        let cpi_context = CpiContext::new(
            self.system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: self.maker.to_account_info(),
                to: escrow_account.to_account_info(),
            },
        );

        anchor_lang::system_program::transfer(cpi_context, lamports)?;

        // Update escrow balance
        escrow_account.balance += lamports;

        Ok(())
    }
}