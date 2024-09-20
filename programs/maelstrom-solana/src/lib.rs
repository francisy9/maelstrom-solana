use anchor_lang::prelude::*;

declare_id!("GtyMKXVfuP53hucpw33jTakP6JWvA3TuSTZ2wkTCsweG");

#[program]
pub mod maelstrom_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
