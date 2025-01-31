use anchor_lang::prelude::*;

declare_id!("3ssRqnuLbw9yhWj5Mw3eQoyJNscDbNSv6evDXWh5UJu1");

#[program]
pub mod memepool {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
