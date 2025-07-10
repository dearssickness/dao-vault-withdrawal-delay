#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

pub mod instructions;
pub mod state;
pub mod errors;

use state::*;
use instructions::*;

declare_id!("BtsLYyDhR5fGQXYv61qiG8wPpervVP4MmNqQG43LdsmR");

#[program]
pub mod dao_vault_withdrawal_delay {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
