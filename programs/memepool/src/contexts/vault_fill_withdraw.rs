use anchor_lang::prelude::*;

use crate::{constants::AGGREGATOR_BOT, errors::VaultError, state::{Vault, WithdrawRequest}};

#[derive(Accounts)]
pub struct VaultFillWithdraw<'info> {
    // Aggregator signer
    #[account(mut, address=AGGREGATOR_BOT)]
    pub aggregator: Signer<'info>,

    /// CHECK: This is the withdrawer’s public key, only used for PDA seeds, no data validation needed.
    #[account()]
    pub withdrawer: AccountInfo<'info>,

    #[account(
        mut,
        seeds=[b"withdraw_request", withdrawer.key().as_ref(), &withdraw_request.count.to_le_bytes()],
        bump=withdraw_request.bump,
    )]
    pub withdraw_request: Account<'info, WithdrawRequest>,
    
    #[account(
        mut,
        seeds=[b"vault".as_ref()],
        bump=vault.bump,
    )]
    pub vault: Account<'info, Vault>,

    pub system_program: Program<'info, System>,
}

impl<'info> VaultFillWithdraw<'info> {
    pub fn vault_fill_withdraw(&mut self, fill_lamports: u64) -> Result<()> {
        // Withdraw Request Account status must be 0 (NOT ready)
        require!(
            self.withdraw_request.status == 0,
            VaultError::WithdrawRequestReady,
        );

        // Transfer $SOL from vault to withdraw request account
        **(self.vault.to_account_info()).try_borrow_mut_lamports()? -= fill_lamports;
        **(self.withdraw_request.to_account_info()).try_borrow_mut_lamports()? += fill_lamports;

        // Update vault.lamport_value
        self.vault.lamports -= fill_lamports;

        // Set withdraw request status to ready
        self.withdraw_request.status = 1;

        Ok(())
    }
}