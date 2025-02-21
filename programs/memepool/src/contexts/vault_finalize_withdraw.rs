use anchor_lang::prelude::*;

use crate::{errors::VaultError, state::WithdrawRequest};

#[derive(Accounts)]
pub struct FinalizeWithdrawVault<'info> {
    #[account(mut)]
    pub withdrawer: Signer<'info>,

    #[account(
        mut,
        close=withdrawer,
        seeds=[b"withdraw_request", withdrawer.key().as_ref(), &withdraw_request.count.to_le_bytes()],
        bump=withdraw_request.bump,
    )]
    pub withdraw_request: Account<'info, WithdrawRequest>,

    // TODO: Do I need this?
    pub system_program: Program<'info, System>,
}

impl<'info> FinalizeWithdrawVault<'info> {
    pub fn vault_finalize_withdraw(&self) -> Result<()> {
        // Withdraw Request Account status must be 1 (ready)
        require!(
            self.withdraw_request.status == 1,
            VaultError::WithdrawRequestNotReady,
        );

        Ok(())
    }
}