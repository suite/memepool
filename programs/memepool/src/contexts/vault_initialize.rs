use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

use crate::state::vault::Vault;

#[derive(Accounts)]
pub struct VaultInitialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer=admin,
        seeds=[b"vault"],
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
        seeds=[b"meme"], // Do I really need to pass in vault.key().as_ref() here? (omitted for now)
        bump,
        mint::decimals=9,
        mint::authority=vault,
    )]
    pub meme_mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

impl<'info> VaultInitialize<'info> {
    pub fn vault_initialize(&mut self, bumps: &VaultInitializeBumps) -> Result<()> {
        self.vault.set_inner(Vault {
            meme_bump: bumps.meme_mint,
            bump: bumps.vault,
            lamports: 0,
            available_lamports: 0,
        });

        Ok(())
    }
}