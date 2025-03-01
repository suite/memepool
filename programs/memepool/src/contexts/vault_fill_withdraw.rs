use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{burn, close_account, transfer, Burn, CloseAccount, Mint, Token, TokenAccount, Transfer}};

use crate::{constants::AGGREGATOR_BOT, errors::VaultError, state::{Vault, WithdrawRequest}};

#[derive(Accounts)]
pub struct VaultFillWithdraw<'info> {
    // Aggregator signer
    #[account(mut, address=AGGREGATOR_BOT)]
    pub aggregator: Signer<'info>,

    // TODO: Might not need? Could use withdraw_request.user for PDA in withdraw_request
    /// CHECK: This is the withdrawer’s public key, only used for PDA seeds, no data validation needed.
    #[account()]
    pub withdrawer: AccountInfo<'info>,

    #[account(
        mut,
        seeds=[b"withdraw_request", withdrawer.key().as_ref(), &withdraw_request.count.to_le_bytes()],
        bump=withdraw_request.bump,
    )]
    pub withdraw_request: Box<Account<'info, WithdrawRequest>>,

    #[account(
        mut,
        associated_token::mint = meme_mint,
        associated_token::authority = withdraw_request,
    )]
    pub withdraw_request_meme_ata: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = wsol_mint,
        associated_token::authority = vault,
    )]
    pub vault_wsol_ata: Account<'info, TokenAccount>,

    // We need a temporary wsol ata to only swap specific amount of WSOL into SOL
    // We use withdraw_request as a "temp" authority, we will close this in the same request
    #[account(
        init,
        payer = aggregator,
        associated_token::mint = wsol_mint,
        associated_token::authority = withdraw_request,
    )]
    pub temp_vault_wsol_ata: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        seeds=[b"vault"],
        bump=vault.bump,
    )]
    pub vault: Box<Account<'info, Vault>>,

    #[account(
        mut,
        seeds=[b"meme"],
        bump=vault.meme_bump,
    )]
    pub meme_mint: Box<Account<'info, Mint>>,

    pub wsol_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> VaultFillWithdraw<'info> {
    pub fn vault_fill_withdraw(&mut self, fill_lamports: u64) -> Result<()> {
        // Withdraw Request Account status must be 0 (NOT ready)
        require!(
            self.withdraw_request.status == 0,
            VaultError::WithdrawRequestReady,
        );

        // Burn $MEME inside withdraw request account
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Burn {
            mint: self.meme_mint.to_account_info(),
            from: self.withdraw_request_meme_ata.to_account_info(),
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
        burn(cpi_ctx, self.withdraw_request.meme_amt)?;

        // Unwrap WSOL
        // 1. Transfer SOL from vault to temp WSOL account
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.vault_wsol_ata.to_account_info(),
            to: self.temp_vault_wsol_ata.to_account_info(),
            authority: self.vault.to_account_info(),
        };
        let seeds = &[
            b"vault".as_ref(),
            &[self.vault.bump],
        ];
        let signer_seeds = &[&seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        transfer(cpi_ctx, fill_lamports)?;
        
        // 2. Close account into vault to retrieve SOL
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = CloseAccount {
            account: self.temp_vault_wsol_ata.to_account_info(),
            destination: self.vault.to_account_info(),
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

        // NOTE: THIS MUST BE THE LAST TRANSACTION
        // Transfer $SOL from vault to withdraw request account
        **(self.vault.to_account_info()).try_borrow_mut_lamports()? -= fill_lamports;
        **(self.withdraw_request.to_account_info()).try_borrow_mut_lamports()? += fill_lamports;

        // Update vault.lamport_value
        self.vault.lamports -= fill_lamports;
        self.vault.available_lamports -= fill_lamports;

        // Set withdraw request status to ready
        self.withdraw_request.status = 1;

        Ok(())
    }
}