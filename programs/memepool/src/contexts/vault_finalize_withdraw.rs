use anchor_lang::prelude::*;
use anchor_spl::token::{close_account, CloseAccount, Mint, Token, TokenAccount};

use crate::{errors::VaultError, state::{Vault, WithdrawRequest}};

#[derive(Accounts)]
pub struct VaultFinalizeWithdraw<'info> {
    #[account(mut)]
    pub withdrawer: Signer<'info>,

    #[account(
        mut,
        associated_token::mint = meme_mint,
        associated_token::authority = withdraw_request,
    )]
    pub withdraw_request_meme_ata: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        close=withdrawer,
        seeds=[b"withdraw_request", withdrawer.key().as_ref(), &withdraw_request.count.to_le_bytes()],
        bump=withdraw_request.bump,
    )]
    pub withdraw_request: Box<Account<'info, WithdrawRequest>>,

    #[account(
        seeds=[b"vault"],
        bump=vault.bump,
    )]
    pub vault: Box<Account<'info, Vault>>,

    #[account(
        seeds=[b"meme"],
        bump=vault.meme_bump,
    )]
    pub meme_mint: Box<Account<'info, Mint>>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

impl<'info> VaultFinalizeWithdraw<'info> {
    pub fn vault_finalize_withdraw(&self) -> Result<()> {
        // Withdraw Request Account status must be 1 (ready)
        require!(
            self.withdraw_request.status == 1,
            VaultError::WithdrawRequestNotReady,
        );

        // Explicitly close withdraw_request_meme_ata
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = CloseAccount {
            account: self.withdraw_request_meme_ata.to_account_info(),
            destination: self.withdrawer.to_account_info(),
            authority: self.withdraw_request.to_account_info(),
        };
        let seeds = &[
            b"withdraw_request",
            self.withdrawer.to_account_info().key.as_ref(),
            &self.withdraw_request.count.to_le_bytes(),
            &[self.withdraw_request.bump],
        ];
        let signer_seeds = &[&seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        close_account(cpi_ctx)?;

        Ok(())
    }
}