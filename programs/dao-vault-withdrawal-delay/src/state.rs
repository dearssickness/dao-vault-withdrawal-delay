use anchor_lang::prelude::*;

#[account]
pub struct Multisig{
    pub signers: Vec<Pubkey>,
    pub threshold: u8,
    pub approvals: u64,
    pub initialized: bool,
}