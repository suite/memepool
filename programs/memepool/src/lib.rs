use anchor_lang::prelude::*;

declare_id!("CNjsRqZKwi66nVBRcgpQVzHBwFBemoboTQj97TFuJRQY");

pub mod constants;
mod contexts;
pub mod errors;
mod state;
pub mod utils;

pub use contexts::*;

#[program]
pub mod memepool {
    use super::*;

    pub fn vault_initialize(ctx: Context<VaultInitialize>) -> Result<()> {
        ctx.accounts.vault_initialize(&ctx.bumps)
    }

    pub fn vault_deposit(ctx: Context<VaultDeposit>, deposit_lamports: u64) -> Result<()> {
        ctx.accounts.vault_deposit(deposit_lamports)
    }

    pub fn vault_request_withdraw(ctx: Context<VaultRequestWithdraw>, meme_amt: u64) -> Result<()> {
        ctx.accounts.vault_request_withdraw(meme_amt, &ctx.bumps)
    }

    pub fn vault_finalize_withdraw(ctx: Context<VaultFinalizeWithdraw>) -> Result<()> {
        ctx.accounts.vault_finalize_withdraw()
    }

    pub fn vault_fill_withdraw(ctx: Context<VaultFillWithdraw>, fill_lamports: u64) -> Result<()> {
        ctx.accounts.vault_fill_withdraw(fill_lamports)
    }

    pub fn lp_deposit(
        ctx: Context<LpDeposit>,
        lp_token_amount: u64,
        maximum_token_0_amount: u64,
        maximum_token_1_amount: u64,
    ) -> Result<()> {
        ctx.accounts.lp_deposit(
            lp_token_amount,
            maximum_token_0_amount,
            maximum_token_1_amount,
            &ctx.bumps,
        )
    }

    pub fn lp_withdraw(
        ctx: Context<LpWithdraw>,
        lp_token_amount: u64,
        minimum_token_0_amount: u64,
        minimum_token_1_amount: u64,
    ) -> Result<()> {
        ctx.accounts.lp_withdraw(
            lp_token_amount,
            minimum_token_0_amount,
            minimum_token_1_amount,
        )
    }

    pub fn lp_swap(ctx: Context<LpSwap>, amount_in: u64, minimum_amount_out: u64) -> Result<()> {
        ctx.accounts.lp_swap(amount_in, minimum_amount_out)
    }
}
