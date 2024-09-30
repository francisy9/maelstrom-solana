use anchor_lang::prelude::*;
use anchor_spl::token::{Token, Mint};
use crate::program::MaelstromSolana;

pub fn initialize_token_mint(_: Context<InitializeTokenMint>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeTokenMint<'info> {
    #[account(constraint = program.programdata_address()? == Some(program_data.key()))]
    pub program: Program<'info, MaelstromSolana>,
    #[account(constraint = program_data.upgrade_authority_address == Some(authority.key()))]
    pub program_data: Account<'info, ProgramData>,
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account( 
        init,
        payer = authority, 
        mint::decimals = 0,
        mint::authority = authority,
        mint::freeze_authority = authority,
        seeds = [b"mint"], 
        bump,
    )]
    pub mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}