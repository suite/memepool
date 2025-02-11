use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct VaultPool {
    pub bump: u8,
    pub pool_id: Pubkey,
}