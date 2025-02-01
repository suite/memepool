use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct WithdrawVault<'info> {
    #[account(mut)]
    pub withdrawer: Signer<'info>,
}

impl<'info> WithdrawVault<'info> {
    pub fn withdraw_vault() -> Result<()> {
        Ok(())
    }
}