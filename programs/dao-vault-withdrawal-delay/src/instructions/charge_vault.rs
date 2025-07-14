use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use crate::{constants::*, errors::*, state::*};

#[derive(Accounts)]
pub struct ChargeVault<'info> {
    #[account(
        mut,
        seeds = [b"vault", dao_multisig.key().as_ref()],
        bump,
    )]
    pub vault: Account<'info, TokenAccount>,
    
    #[account(
        seeds = [b"dao_multisig"],
        bump,
    )]
    pub dao_multisig: Account<'info, Multisig>,

    #[account(mut)]
    pub admin_wallet: Account<'info, TokenAccount>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub token_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>
}
    
pub fn handler(ctx: Context<ChargeVault>, amount: u64) -> Result<()> {
    
    let token_program = ctx.accounts.token_program.to_account_info();
    
    let transfer = token::Transfer {
        from: ctx.accounts.admin_wallet.to_account_info(),
        to: ctx.accounts.vault.to_account_info(),
        authority: ctx.accounts.admin.to_account_info()
    };
    
    let cpi_ctx = CpiContext::new(token_program, transfer);

    token::transfer(cpi_ctx, amount)?;

    Ok(())
}