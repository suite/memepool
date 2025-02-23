use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, token::Token, token_2022::Token2022, token_interface::{Mint, TokenAccount}
};
use raydium_cpmm_cpi::{cpi, program::RaydiumCpmm, states::PoolState};

use crate::{constants::AGGREGATOR_BOT, state::{Vault, VaultPool}};

// 
// Deposit into LP for a given pair/raydium id
// TX must be signed by aggregator bot
//

#[derive(Accounts)]
pub struct LpDeposit<'info> {
    #[account(
        init_if_needed,
        payer=aggregator,
        seeds=[b"vault_pool", pool_state.key().as_ref()],
        bump,
        space=8+VaultPool::INIT_SPACE,
    )]
    pub vault_pool: Account<'info, VaultPool>,

    #[account(
        mut,
        seeds=[b"vault".as_ref()],
        bump=vault.bump,
    )]
    pub vault: Account<'info, Vault>,

    // Aggregator signer
    #[account(mut, address=AGGREGATOR_BOT)]
    pub aggregator: Signer<'info>,

    pub cp_swap_program: Program<'info, RaydiumCpmm>,

    /// CHECK: pool vault and lp mint authority
    #[account(
        seeds = [
            raydium_cpmm_cpi::AUTH_SEED.as_bytes(),
        ],
        seeds::program = cp_swap_program.key(),
        bump,
    )]
    pub authority: UncheckedAccount<'info>,

    #[account(mut)]
    pub pool_state: AccountLoader<'info, PoolState>,

    /// Owner lp token account
    #[account(
        init_if_needed,
        payer=aggregator,
        associated_token::mint = lp_mint,
        associated_token::authority = vault
    )]
    pub owner_lp_token: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The payer's token account for token_0
    #[account(
        mut,
        token::mint = token_0_vault.mint,
        token::authority = vault
    )]
    pub token_0_account: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The payer's token account for token_1
    #[account(
        mut,
        token::mint = token_1_vault.mint,
        token::authority = vault
    )]
    pub token_1_account: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The address that holds pool tokens for token_0
    #[account(
        mut,
        constraint = token_0_vault.key() == pool_state.load()?.token_0_vault
    )]
    pub token_0_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The address that holds pool tokens for token_1
    #[account(
        mut,
        constraint = token_1_vault.key() == pool_state.load()?.token_1_vault
    )]
    pub token_1_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    /// token Program
    pub token_program: Program<'info, Token>,

    /// Token program 2022
    pub token_program_2022: Program<'info, Token2022>,

    /// The mint of token_0 vault
    #[account(
        address = token_0_vault.mint
    )]
    pub vault_0_mint: Box<InterfaceAccount<'info, Mint>>,

    /// The mint of token_1 vault
    #[account(
        address = token_1_vault.mint
    )]
    pub vault_1_mint: Box<InterfaceAccount<'info, Mint>>,

    /// Lp token mint
    #[account(
        mut,
        address = pool_state.load()?.lp_mint)
    ]
    pub lp_mint: Box<InterfaceAccount<'info, Mint>>,

    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}


impl<'info> LpDeposit<'info> {
    pub fn lp_deposit(&mut self, lp_token_amount: u64, maximum_token_0_amount: u64, maximum_token_1_amount: u64, bumps: &LpDepositBumps) -> Result<()> {
        let cpi_program = self.cp_swap_program.to_account_info();
        let cpi_accounts = cpi::accounts::Deposit {
            owner: self.vault.to_account_info(), // must be signer?
            authority: self.authority.to_account_info(),
            pool_state: self.pool_state.to_account_info(),
            owner_lp_token: self.owner_lp_token.to_account_info(),
            token_0_account: self.token_0_account.to_account_info(),
            token_1_account: self.token_1_account.to_account_info(),
            token_0_vault: self.token_0_vault.to_account_info(),
            token_1_vault: self.token_1_vault.to_account_info(),
            token_program: self.token_program.to_account_info(),
            token_program_2022: self.token_program_2022.to_account_info(),
            vault_0_mint: self.vault_0_mint.to_account_info(),
            vault_1_mint: self.vault_1_mint.to_account_info(),
            lp_mint: self.lp_mint.to_account_info(),
        };

        let seeds = &[
            b"vault".as_ref(),
            &[self.vault.bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts,signer_seeds);
        cpi::deposit(cpi_context, lp_token_amount, maximum_token_0_amount, maximum_token_1_amount)?;

        // Keep track of pool, only set once TODO: make sure this is safe
        if self.vault_pool.to_account_info().data_is_empty() {
            self.vault_pool.set_inner(VaultPool {
                bump: bumps.vault_pool,
                pool_id: self.pool_state.key(),
            });
        }
        
        Ok(())
    }
}