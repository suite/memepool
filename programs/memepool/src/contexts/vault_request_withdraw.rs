use anchor_lang::prelude::*;
use anchor_spl::token::{burn, Burn, Mint, Token, TokenAccount};

use crate::state::{Portfolio, Vault, WithdrawRequest};

#[derive(Accounts)]
pub struct RequestWithdrawVault<'info> {
    #[account(mut)]
    pub withdrawer: Signer<'info>,

    #[account(
        mut,
        seeds=[b"vault".as_ref()],
        bump=vault.bump,
    )]
    pub vault: Account<'info, Vault>,

    #[account(
        mut,
        seeds=[b"meme".as_ref()],
        bump=vault.meme_bump,
    )]
    pub meme_mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = meme_mint,
        associated_token::authority = withdrawer,
    )]
    pub withdrawer_meme_ata: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer=withdrawer,
        seeds=[b"portfolio", withdrawer.key().as_ref()],
        bump,
        space=8+Portfolio::INIT_SPACE
    )]
    pub portfolio: Account<'info, Portfolio>,

    #[account(
        init,
        payer=withdrawer,
        seeds=[b"withdraw_request", withdrawer.key().as_ref(), &portfolio.counter.to_le_bytes()],
        bump,
        space=8+Portfolio::INIT_SPACE
    )]
    pub withdraw_request: Account<'info, WithdrawRequest>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> RequestWithdrawVault<'info> {
    pub fn vault_request_withdraw(&mut self, meme_amt: u64, bumps: &RequestWithdrawVaultBumps) -> Result<()> {
        // Burn withdrawer's $MEME
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Burn {
            mint: self.meme_mint.to_account_info(),
            from: self.withdrawer_meme_ata.to_account_info(),
            authority: self.withdrawer.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        burn(cpi_ctx, meme_amt)?;

    
        // Create Withdraw Request Account
        self.withdraw_request.set_inner(WithdrawRequest {
            user: self.withdrawer.key(),
            bump: bumps.withdraw_request,
            status: 0,
            meme_amt,
            count: self.portfolio.counter,
        });

        // Init/Update withdrawer's Portfolio Account
        if self.portfolio.counter == 0 {
            self.portfolio.set_inner(Portfolio {
                user: self.withdrawer.key(),
                counter: 1,
                bump: bumps.portfolio,
            });
        } else {
            self.portfolio.counter += 1; // TODO: Maybe do checked add
        }

        Ok(())
    }
}