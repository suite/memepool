use anchor_lang::prelude::*;
use crate::errors::VaultError;

pub fn calculate_meme_from_sol(
    deposit_lamports: u64,
    meme_supply: u64,
    vault_lamports: u64,
) -> Result<u64> {
    if meme_supply == 0 {
        Ok(deposit_lamports)
    } else {
        deposit_lamports
            .checked_mul(meme_supply)
            .and_then(|v| v.checked_div(vault_lamports))
            .ok_or(VaultError::InvalidMEMEAmount.into())
    }
}


// TODO: deprecated
pub fn calculate_sol_from_meme(
    withdraw_meme_amt: u64,
    meme_supply: u64,
    vault_supply: u64,
) -> Result<u64> {
    // TODO: should we do any err checking here? ex vault_supply>0?
    withdraw_meme_amt
        .checked_mul(vault_supply)
        .and_then(|v| v.checked_div(meme_supply))
        .ok_or(VaultError::InvalidSOLAmount.into())
}