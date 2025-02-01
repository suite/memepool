use anchor_lang::prelude::*;
use crate::errors::VaultError;

pub fn calculate_meme_from_sol(
    deposit_lamports: u64,
    meme_supply: u64,
    vault_supply: u64,
) -> Result<u64> {
    if meme_supply == 0 {
        Ok(deposit_lamports)
    } else {
        deposit_lamports
            .checked_mul(meme_supply)
            .and_then(|v| v.checked_div(vault_supply))
            .ok_or(VaultError::InvalidMEMEAmount.into())
    }
}

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

pub fn get_vault_supply(total_lamports: u64, rent: &Rent, space: usize) -> Result<u64> {
    let rent_exempt_minimum = rent.minimum_balance(space);
    total_lamports
        .checked_sub(rent_exempt_minimum)
        .ok_or(VaultError::InvalidVault.into())
} 