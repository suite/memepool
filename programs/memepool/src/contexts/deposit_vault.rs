use anchor_lang::{prelude::*, system_program::{self, Transfer}};
use anchor_spl::{associated_token::AssociatedToken, token::{mint_to, Mint, MintTo, Token, TokenAccount}};

use crate::{state::Vault, utils::{calculate_meme_from_sol, get_vault_supply}};

#[derive(Accounts)]
pub struct DepositVault<'info> {
    #[account(mut)]
    pub depositer: Signer<'info>,

    #[account(
        mut,
        seeds=[b"vault".as_ref()],
        bump=vault.bump,
    )]
    pub vault: Account<'info, Vault>,

    #[account(
        mut,
        seeds=[b"meme".as_ref()],
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
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> DepositVault<'info> {
    // sol to meme
    pub fn deposit_vault(&self, deposit_lamports: u64) -> Result<()> {   
        let meme_supply = self.meme_mint.supply;
        
        // TODO: self.vault.get_lamports() may not be correct (locked in lps)
        // Add another field to vault account
        let vault_supply = get_vault_supply(
            self.vault.get_lamports(),
            &self.rent,
            8 + Vault::INIT_SPACE,
        )?;

        let meme_amt = calculate_meme_from_sol(deposit_lamports, meme_supply, vault_supply)?;

        // Transfer SOL from user to vault
        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.depositer.to_account_info(),
            to: self.vault.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        system_program::transfer(cpi_ctx, deposit_lamports)?;

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