use anchor_lang::prelude::*;
use crate::{state::user_collection::*, constants::*, errors::MaelstromSolanaError::CardIdTooLarge};

pub fn increment_card_count(ctx: Context<IncrementCardCount>, card_update_argument: Vec<CardUpdateStruct>) -> Result<()> {
    let user_collection = &mut ctx.accounts.user_collection;
    for card_update in card_update_argument {
        if card_update.card_id > MAX_CARD_ID {
            return Err(CardIdTooLarge.into());
        }

        let byte_array_index: usize = (card_update.card_id / 2) as usize;

        // TODO: Handle overflow in future
        if card_update.card_id % 2 == 0 {
            let current_card_count = (user_collection.card_count[byte_array_index] & 240) >> 4;
            if !(current_card_count + card_update.amount > 15) {
                user_collection.card_count[byte_array_index] += card_update.amount << 4;
            }
        } else {
            let current_card_count = user_collection.card_count[byte_array_index] & 15;
            if !(current_card_count + card_update.amount > 15) {
                user_collection.card_count[byte_array_index] += card_update.amount;
            }
        }
    }
    Ok(())
}

#[derive(Default, AnchorSerialize, AnchorDeserialize)]
pub struct CardUpdateStruct {
    pub card_id: u8,
    pub amount: u8,
}

#[derive(Accounts)]
pub struct IncrementCardCount<'info> {
    /// CHECK: This is a user account used to check seed
    user: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [
            b"user-collection", 
            user.key().as_ref(), 
            user_collection.collection_id.to_le_bytes().as_ref()
        ],  bump = user_collection.bump)]
    user_collection: Account<'info, UserCollection>,
    #[account(mut, address = CARD_UPDATE_AUTHORITY)]
    collection_update_authority: Signer<'info>,
    system_program: Program<'info, System>
}