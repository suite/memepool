use anchor_lang::prelude::*;

/*
 * Vault
 * Only one allowed, created once
 * Holds SOL to be used in LPS
 * Owns $MEME mint
 */
#[account]
#[derive(InitSpace)]
pub struct Vault {
    pub bump: u8,
}
