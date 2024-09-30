use anchor_lang::prelude::*;
use instructions::*;

pub mod instructions;
pub mod errors;

declare_id!("CGBsbMYVnmMEgcZu19xYKc5vjVCBWzZ3q1XHdY37jftW");

#[program]
pub mod maelstrom_solana {
    use super::*;

    pub fn initialize_token_mint(ctx: Context<InitializeTokenMint>) -> Result<()> {
        instructions::initialize_token_mint(ctx)
    }
}