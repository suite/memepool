use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{burn, transfer, Burn, Mint, Token, TokenAccount, Transfer}};

use crate::state::{Portfolio, Vault, WithdrawRequest};

#[derive(Accounts)]
pub struct VaultRequestWithdraw<'info> {
    #[account(mut)]
    pub withdrawer: Signer<'info>,

    #[account(
        mut,
        seeds=[b"vault"],
        bump=vault.bump,
    )]
    pub vault: Box<Account<'info, Vault>>,

    #[account(
        mut,
        seeds=[b"meme"],
        bump=vault.meme_bump,
    )]
    pub meme_mint: Box<Account<'info, Mint>>,

    #[account(
        mut,
        associated_token::mint = meme_mint,
        associated_token::authority = withdrawer,
    )]
    pub withdrawer_meme_ata: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer=withdrawer,
        seeds=[b"portfolio", withdrawer.key().as_ref()],
        bump,
        space=8+Portfolio::INIT_SPACE
    )]
    pub portfolio: Box<Account<'info, Portfolio>>,

    #[account(
        init,
        payer=withdrawer,
        seeds=[b"withdraw_request", withdrawer.key().as_ref(), &portfolio.counter.to_le_bytes()],
        bump,
        space=8+WithdrawRequest::INIT_SPACE
    )]
    pub withdraw_request: Box<Account<'info, WithdrawRequest>>,

    #[account(
        init_if_needed,
        payer=withdrawer,
        associated_token::mint = meme_mint,
        associated_token::authority = withdraw_request,
    )]
    pub withdraw_request_meme_ata: Box<Account<'info, TokenAccount>>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> VaultRequestWithdraw<'info> {
    pub fn vault_request_withdraw(&mut self, meme_amt: u64, bumps: &VaultRequestWithdrawBumps) -> Result<()> {
        // Create Withdraw Request Account
        self.withdraw_request.set_inner(WithdrawRequest {
            user: self.withdrawer.key(),
            bump: bumps.withdraw_request,
            status: 0,
            meme_amt,
            count: self.portfolio.counter,
        });
      
        // Transfer $MEME to withdraw_request, burn on fill
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.withdrawer_meme_ata.to_account_info(),
            to: self.withdraw_request_meme_ata.to_account_info(),
            authority: self.withdrawer.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer(cpi_ctx, meme_amt)?;

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