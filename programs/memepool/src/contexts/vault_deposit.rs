use anchor_lang::{prelude::*, system_program::{self, Transfer}};
use anchor_spl::{associated_token::AssociatedToken, token::{mint_to, Mint, MintTo, Token, TokenAccount}};

use crate::{state::Vault, utils::calculate_meme_from_sol};

#[derive(Accounts)]
pub struct VaultDeposit<'info> {
    #[account(mut)]
    pub depositer: Signer<'info>,

    #[account(
        mut,
        seeds=[b"vault"],
        bump=vault.bump,
    )]
    pub vault: Account<'info, Vault>,

    #[account(
        mut,
        seeds=[b"meme"],
        bump=vault.meme_bump,
    )]
    pub meme_mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = depositer,
        associated_token::mint = meme_mint,
        associated_token::authority = depositer,
    )]
    pub depositer_meme_ata: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> VaultDeposit<'info> {
    // $SOL to $MEME
    pub fn vault_deposit(&mut self, deposit_lamports: u64) -> Result<()> {
        // Calculate amount of $MEME to mint
        let meme_supply = self.meme_mint.supply;
        let vault_lamports = self.vault.lamports;
        let meme_amt = calculate_meme_from_sol(deposit_lamports, meme_supply, vault_lamports)?;

        // Transfer SOL from user to vault
        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.depositer.to_account_info(),
            to: self.vault.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        system_program::transfer(cpi_ctx, deposit_lamports)?;

        // Update vault.lamport_value
        self.vault.lamports += deposit_lamports;

        // Mint $MEME
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = MintTo {
            mint: self.meme_mint.to_account_info(),
            to: self.depositer_meme_ata.to_account_info(),
            authority: self.vault.to_account_info(),
        };
        let seeds = &[
            b"vault".as_ref(),
            &[self.vault.bump],
        ];
        let signer_seeds = &[&seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        mint_to(cpi_ctx, meme_amt)?;

        Ok(())
    }
}