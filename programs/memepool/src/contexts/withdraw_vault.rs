use anchor_lang::prelude::*;
use anchor_spl::token::{burn, Burn, Mint, Token, TokenAccount};

use crate::{state::Vault, utils::{calculate_sol_from_meme, get_vault_supply}};

#[derive(Accounts)]
pub struct WithdrawVault<'info> {
    #[account(mut)]
    pub withdrawer: Signer<'info>,

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
        mut,
        token::mint = meme_mint,
        token::authority = withdrawer, // token:: here or ata used in deposit?
    )]
    pub withdrawer_meme_ata: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> WithdrawVault<'info> {
    // meme to sol
    pub fn withdraw_vault(&self, withdraw_meme_amt: u64) -> Result<()> {
        let meme_supply = self.meme_mint.supply;
        
        let vault_supply = get_vault_supply(
            self.vault.get_lamports(),
            &self.rent,
            8 + Vault::INIT_SPACE,
        )?;

        let sol_amt = calculate_sol_from_meme(withdraw_meme_amt, meme_supply, vault_supply)?;
        
        // Burn user's $MEME - only needs withdrawer's signature
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Burn {
            mint: self.meme_mint.to_account_info(),
            from: self.withdrawer_meme_ata.to_account_info(),
            authority: self.withdrawer.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        burn(cpi_ctx, withdraw_meme_amt)?;

        // Transfer SOL from vault PDA to user
        **(self.vault.to_account_info()).try_borrow_mut_lamports()? -= sol_amt;
        **(self.withdrawer.to_account_info()).try_borrow_mut_lamports()? += sol_amt;
        
        Ok(())
    }
}