use anchor_lang::prelude::*;

#[account]
pub struct Multisig {
    pub recipient: Pubkey,
    pub signers: Vec<Pubkey>,
    pub threshold: u8,
    pub approvals: u64,
    pub initialized: bool,
    pub unlock_at: i64,
    pub delay: u64
}