use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct WithdrawRequest {
    pub user: Pubkey,
    pub bump: u8,
    pub status: u8,
    pub meme_amt: u64,
    pub count: u64, // TODO: could have #[instruction(counter: u64)] inside FinalizeWithdrawVault instead, TBD if user will know count
}
