use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use crate::{constants::*, errors::*, state::*};

#[derive(Accounts)]
pub struct SetMultisig<'info> {
    #[account(
        mut,
        seeds = [b"dao_multisig"],
        bump,
    )]
    pub dao_multisig: Account<'info, Multisig>,

    #[account(mut)]
    pub admin: Signer<'info>,
}

pub fn handler(
    ctx: Context<SetMultisig>, 
    signers: Vec<Pubkey>, 
    recipient: Pubkey, 
    threshold: u8,
    delay: u64
) -> Result<()> {

    require!(
        threshold > 0 && threshold <= MAXIMUM_SIGNERS as u8,
        MultisigError::InvalidThreshold
    );
    
    require!(signers.len() <= MAXIMUM_SIGNERS, MultisigError::TooManySigners);

    let multisig = &mut ctx.accounts.dao_multisig;
    multisig.recipient = recipient;
    multisig.signers = signers;
    multisig.threshold = threshold;
    multisig.approvals = 0;
    multisig.initialized = true;
    multisig.unlock_at = 0;
    multisig.delay = delay;

    Ok(())
} 