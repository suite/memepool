use anchor_lang::prelude::*;

use crate::constants::AGGREGATOR_BOT;

#[derive(Accounts)]
pub struct VaultFillWithdraw<'info> {
    #[account(mut, address=AGGREGATOR_BOT)]
    pub aggregator: Signer<'info>,
}

impl<'info> VaultFillWithdraw<'info> {
    // transfer 
}