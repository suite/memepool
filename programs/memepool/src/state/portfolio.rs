use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Portfolio {
    pub user: Pubkey,
    pub counter: u64,
    pub bump: u8,
}
