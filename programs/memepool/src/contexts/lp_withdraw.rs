use anchor_lang::prelude::*;
use raydium_cpmm_cpi::{cpi, program::RaydiumCpmm};

use crate::{constants::AGGREGATOR_BOT, state::{Vault, VaultPool}};

#[derive(Accounts)] 
pub struct WithdrawLp<'info> {
    // Vault will pay for all txs (TODO doesnt right now, fix )
    #[account(
        mut,
        seeds=[b"vault".as_ref()],
        bump=vault.bump,
    )]
    pub vault: Account<'info, Vault>,

    // Aggregator signer
    #[account(mut, address=AGGREGATOR_BOT)]
    pub aggregator: Signer<'info>,

    // TODO: Take in all fields necessary similar to deposit_lp

    pub cp_swap_program: Program<'info, RaydiumCpmm>,

    pub system_program: Program<'info, System>,
}

impl<'info> WithdrawLp<'info> {
    
    pub fn withdraw_lp(&self, amount: u64) -> Result<()> {
        // Fill out similar to deposit_lp.rs
        Ok(())
    }
}