use anchor_lang::prelude::*;

#[account]
pub struct UserCollection {
    pub collection_id: u8,
    pub gold: u16,
    pub card_count: [u8; 100],
    pub bump: u8,
}