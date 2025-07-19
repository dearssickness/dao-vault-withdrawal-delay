use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use crate::{constants::*, errors::*, state::*};


#[derive(Accounts)]
pub struct Execute<'info> {
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
    
    /// CHECK: This account just used to sign transfers from vesting_vault
    #[account(
        seeds = [b"authority", token_mint.key().as_ref()],
        bump,
    )]
    pub authority: AccountInfo<'info>,

    #[account(mut)]
    pub recipient: Account<'info, TokenAccount>,

    pub token_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>

}

pub fn handler(ctx: Context<Execute>) -> Result<()> {
    let amount  = ctx.accounts.vault.amount;

    let token_mint_key = ctx.accounts.token_mint.key();

    let authority_seeds = &[
        b"authority",
        token_mint_key.as_ref(),
        &[ctx.bumps.authority]
    ];
    let signer = &[&authority_seeds[..]];

    let token_program = ctx.accounts.token_program.to_account_info();
    
    let transfer  = token::Transfer {
        from: ctx.accounts.vault.to_account_info(),
        to: ctx.accounts.recipient.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    
    let cpi_ctx = CpiContext::new_with_signer(token_program, transfer, signer);
    
    token::transfer(cpi_ctx, amount)?;

    Ok(())
}