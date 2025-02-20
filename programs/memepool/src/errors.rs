use anchor_lang::error_code;

#[error_code]
pub enum VaultError {
    #[msg("Error calculating $MEME mint amount")]
    InvalidMEMEAmount,
    #[msg("Error calculating $SOL transfer amount")]
    InvalidSOLAmount,
    #[msg("Error calculating vault lamports")]
    InvalidVault,
    #[msg("Vault OUT OF SOL")]
    VaultOOS,
    #[msg("Withdraw Request status not ready")]
    WithdrawRequestNotReady,
}