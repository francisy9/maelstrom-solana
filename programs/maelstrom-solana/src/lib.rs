use anchor_lang::prelude::*;
use instructions::*;

pub mod state;
pub mod instructions;
pub mod errors;
pub mod constants;

declare_id!("3UAp9o6N1YM1XNV8jjG2hGkvF4Rk9PicroTMLJXuzQWJ");

#[program]
pub mod maelstrom_solana {
    use super::*;

    pub fn initialize_token_mint(ctx: Context<InitializeTokenMint>) -> Result<()> {
        instructions::initialize_token_mint(ctx)
    }

    pub fn initialize_user_collection(ctx: Context<InitializeUserCollection>, collection_id: u8) -> Result<()> {
        instructions::initialize_user_collection(ctx, collection_id)
    }

    pub fn increment_card_count(ctx: Context<IncrementCardCount>, card_update_argument: Vec<CardUpdateStruct>) -> Result<()> {
        instructions::increment_card_count(ctx, card_update_argument)
    }
}
