use anchor_lang::error_code;

#[error_code]
pub enum VaultError {
    #[msg("Error calculating $MEME mint amount")]
    InvalidMEMEAmount,
    #[msg("Error calculating vault lamports")]
    InvalidVault,
}