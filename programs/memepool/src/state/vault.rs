use anchor_lang::prelude::*;

/*
 * Vault
 * Only one allowed, created once
 * Holds SOL to be used in LPS
 * Owns $MEME mint
 * Tracks `total_value` in lamports
 */
#[account]
#[derive(InitSpace)]
pub struct Vault {
    pub meme_bump: u8,
    pub bump: u8,
    pub lamports: u64,
}
