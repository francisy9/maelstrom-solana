use anchor_lang::prelude::*;
use crate::{state::user_collection::*, constants::CARD_UPDATE_AUTHORITY};

pub fn initialize_user_collection(ctx: Context<InitializeUserCollection>, collection_id: u8) -> Result<()> {
    let user_collection = &mut ctx.accounts.user_collection;
    user_collection.gold = 0;
    user_collection.collection_id = collection_id;
    user_collection.bump = ctx.bumps.user_collection;
    Ok(())
}

#[derive(Accounts)]
#[instruction(collection_id: u8)]
pub struct InitializeUserCollection<'info> {
    /// CHECK: This is a user account used to check seed
    pub user: UncheckedAccount<'info>,
    #[account(
        init,
        payer = authority,
        space = 8 + 1 + 2 + 100 + 1, seeds = [b"user-collection", user.key().as_ref(), collection_id.to_le_bytes().as_ref()],  bump)]
    pub user_collection: Account<'info, UserCollection>,
    #[account(mut, address = CARD_UPDATE_AUTHORITY)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>
}