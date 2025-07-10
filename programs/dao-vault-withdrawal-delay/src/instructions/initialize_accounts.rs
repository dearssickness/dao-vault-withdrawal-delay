use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use crate::{errors::*, state::*};

#[derive(Accounts)]
pub struct InitializeAccounts<'info> {
    #[account(
        init,
        payer = admin,
        seeds = [b"vault", dao_multisig.key().as_ref()],
        bump,
        token::mint = token_mint,
        token::authority = authority,
    )]
    pub vault: Account<'info, TokenAccount>,
    
    #[account(
        init,
        payer = admin,
        seeds = [b"dao_multisig"],
        bump,
        space = 1,
    )]
    pub dao_multisig: Account<'info, Multisig>,

    /// CHECK: This account just used to sign transfers from vesting_vault
    #[account(
        seeds = [b"authority", token_mint.key().as_ref()],
        bump,
    )]
    pub authority: AccountInfo<'info>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub token_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>
}

pub fn handler(ctx: Context<InitializeAccounts>) -> Result<()> {
    Ok(())
}