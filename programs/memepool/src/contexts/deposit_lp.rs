use anchor_lang::prelude::*;

use crate::constants::{AGGREGATOR_BOT, RAYDIUM_LIQUIDITY_POOL_V4_PROGRAM_ID};

// 
// Deposit into LP for a given pair/raydium id
// TX must be signed by aggregator bot
//

#[derive(Accounts)]
pub struct DepositLp<'info> {
    #[account(address=AGGREGATOR_BOT)]
    pub aggregator: Signer<'info>,

    /// CHECK: pool state account
    #[account(owner=RAYDIUM_LIQUIDITY_POOL_V4_PROGRAM_ID)]
    pub pool_state: AccountInfo<'info>,

}


impl<'info> DepositLp<'info> {
    pub fn deposit_lp(&self) -> Result<()> {
        let pool_id = self.pool_state.key();
        msg!("pool_id {}", pool_id);


        /*
        ctx: Context<Deposit>,
    lp_token_amount: u64,
    maximum_token_0_amount: u64,
    maximum_token_1_amount: u64,


    Context<Deposit>
    #[account(
        seeds = [
            crate::AUTH_SEED.as_bytes(),
        ],
        bump,
    )]
    authority

    need to get, constant

    /// Owner lp tokan account
    #[account(mut,  token::authority = owner)]
    pub owner_lp_token: Box<InterfaceAccount<'info, TokenAccount>>,

    PDA derived?

         */
    
        Ok(())
    }
}