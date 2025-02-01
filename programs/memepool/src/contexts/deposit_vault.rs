use anchor_lang::{prelude::*, solana_program::{program::invoke, system_instruction::transfer}};
use anchor_spl::{associated_token::AssociatedToken, token::{mint_to, Mint, MintTo, Token, TokenAccount}};

use crate::{errors::VaultError, state::Vault};

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
    pub fn deposit_vault(&self, deposit_lamports: u64) -> Result<()> {
        // Mint $MEME
        let meme_supply = self.meme_mint.supply;

        // vault_supply = vault_lamports - (rent cost for vault account)
        let vault_supply = self.vault.get_lamports();
        let rent_exempt_minimum = self.rent.minimum_balance(8+Vault::INIT_SPACE); // does space=size?\

        let vault_supply = vault_supply
            .checked_sub(rent_exempt_minimum)
            .ok_or(VaultError::InvalidVault)?;

        let ix = transfer(&self.depositer.key(),&self.vault.key(), deposit_lamports);

        invoke(&ix, &[
            self.depositer.to_account_info(),
            self.vault.to_account_info(),
            self.system_program.to_account_info(),
        ])?;

        let meme_amt: u64 = if meme_supply == 0 {
            deposit_lamports
        } else {
            deposit_lamports
                .checked_mul(meme_supply)
                .and_then(|v| v.checked_div(vault_supply))
                .ok_or(VaultError::InvalidMEMEAmount)?
        };

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