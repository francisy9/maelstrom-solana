use anchor_lang::prelude::*;

pub fn initialize_user_collection(ctx: Context<InitializeUserCollection>, collection_id: u8) -> Result<()> {
    let user_collection = &mut ctx.accounts.user_collection;
    user_collection.gold = 0;
    user_collection.collection_id = collection_id;
    user_collection.bump = ctx.bumps.user_collection;
    Ok(())
}

#[account]
pub struct UserCollection {
    collection_id: u8,
    gold: u16,
    card_count: [u8; 100],
    bump: u8,
}

#[derive(Accounts)]
#[instruction(collection_id: u8)]
pub struct InitializeUserCollection<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        space = 8 + 1 + 2 + 100 + 1, seeds = [b"user-collection", user.key().as_ref(), collection_id.to_le_bytes().as_ref()],  bump)]
    pub user_collection: Account<'info, UserCollection>,
    pub system_program: Program<'info, System>
}