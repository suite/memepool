use anchor_lang::prelude::*;

declare_id!("3ssRqnuLbw9yhWj5Mw3eQoyJNscDbNSv6evDXWh5UJu1");

mod state;
mod contexts;

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

*/

#[program]
pub mod memepool {
    use super::*;

    pub fn initialize_vault(ctx: Context<InitializeVault>) -> Result<()> {
        ctx.accounts.initialize_vault(&ctx.bumps)
    }
}