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

    pub fn vault_initialize(ctx: Context<InitializeVault>) -> Result<()> {
        ctx.accounts.vault_initialize(&ctx.bumps)
    }

    pub fn vault_deposit(ctx: Context<DepositVault>, deposit_lamports: u64) -> Result<()> {
        ctx.accounts.vault_deposit(deposit_lamports)
    }

    pub fn vault_request_withdraw(ctx: Context<RequestWithdrawVault>, meme_amt: u64) -> Result<()> {
        ctx.accounts.vault_request_withdraw(meme_amt, &ctx.bumps)
    }

    pub fn vault_finalize_withdraw(ctx: Context<FinalizeWithdrawVault>) -> Result<()> {
        ctx.accounts.vault_finalize_withdraw()
    }

    pub fn lp_deposit(ctx: Context<DepositLp>,  lp_token_amount: u64, maximum_token_0_amount: u64, maximum_token_1_amount: u64) -> Result<()> {
        ctx.accounts.lp_deposit(lp_token_amount, maximum_token_0_amount, maximum_token_1_amount, &ctx.bumps)
    }
}