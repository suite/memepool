use anchor_lang::prelude::*;

declare_id!("4DkcsX8ypqXChJvqUTyVdtL9463KyTT3RBEUHLkTM6Ls");

mod state;
mod contexts;
pub mod errors;
pub mod utils;
pub mod constants;

pub use contexts::*;

/*

define struct
implement easy to test methods

Escrow/Vault world
 - deposit SOL
 - get MEME
    - contexts
    - 

    init
    deposit
    withdraw

Aggregator world
 - deposit into LP using vault funds
 - harvest lp
 - withdraw 

 TODO: 
 Change naming conventions
 deposit_lp -> lp_deposit 
 etc.

*/

#[program]
pub mod memepool {
    use super::*;

    pub fn initialize_vault(ctx: Context<InitializeVault>) -> Result<()> {
        ctx.accounts.initialize_vault(&ctx.bumps)
    }

    pub fn deposit_vault(ctx: Context<DepositVault>, deposit_lamports: u64) -> Result<()> {
        ctx.accounts.deposit_vault(deposit_lamports)
    }

    pub fn request_withdraw_vault(ctx: Context<RequestWithdrawVault>, meme_amt: u64) -> Result<()> {
        ctx.accounts.request_withdraw_vault(meme_amt, &ctx.bumps)
    }

    pub fn finalize_withdraw_vault(ctx: Context<FinalizeWithdrawVault>) -> Result<()> {
        ctx.accounts.finalize_withdraw_vault()
    }

    pub fn deposit_lp(ctx: Context<DepositLp>,  lp_token_amount: u64, maximum_token_0_amount: u64, maximum_token_1_amount: u64) -> Result<()> {
        ctx.accounts.deposit_lp(lp_token_amount, maximum_token_0_amount, maximum_token_1_amount, &ctx.bumps)
    }
}