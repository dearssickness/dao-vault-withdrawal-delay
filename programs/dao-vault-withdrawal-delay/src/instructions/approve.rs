use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use crate::{constants::*, errors::*, state::*};

#[derive(Accounts)]
pub struct Approve<'info>{
    #[account(
        mut,
        seeds = [b"dao_multisig"],
        bump,
    )]
    pub dao_multisig: Account<'info, Multisig>,
    
    #[account(mut)]
    pub user: Signer<'info>,
}

pub fn handler(ctx: Context<Approve>) -> Result<()> {
    let multisig = &mut ctx.accounts.dao_multisig;
    let signer = ctx.accounts.user.key();

    let signer_index = multisig
        .signers
        .iter()
        .position(|&key| key == signer)
        .ok_or(MultisigError::InvalidSigner)?;

    require!(
        (multisig.approvals & (1 << signer_index)) == 0,
        MultisigError::AlreadyApproved
    );

    // Set the approval bit for the signer (e.g., set bit i to 1)
    multisig.approvals |= 1 << signer_index;

    let approval_count = multisig.approvals.count_ones() as u8;
    if approval_count >= multisig.threshold {
        let now = Clock::get()?;
        let delay = multisig.delay;
        multisig.unlock_at = now.unix_timestamp + delay as i64;
        msg!("Threshold met! Action can be executed after {}", &multisig.unlock_at);
    }

    Ok(())
}