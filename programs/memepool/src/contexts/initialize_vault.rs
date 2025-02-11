use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

use crate::state::{vault::Vault, VaultPool};

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer=admin,
        seeds=[b"vault".as_ref()],
        bump,
        space=8+Vault::INIT_SPACE,
    )]
    pub vault: Account<'info, Vault>,

    /*
     * $MEME to represent users stake
     */
    #[account(
        init_if_needed,
        payer=admin,
        seeds=[b"meme".as_ref()], // Do I really need to pass in vault.key().as_ref() here? (omitted for now)
        bump,
        mint::decimals=9,
        mint::authority=vault,
    )]
    pub meme_mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

impl<'info> InitializeVault<'info> {
    pub fn initialize_vault(&mut self, bumps: &InitializeVaultBumps) -> Result<()> {
        self.vault.set_inner(Vault {
            meme_bump: bumps.meme_mint,
            bump: bumps.vault
        });

        Ok(())
    }
}