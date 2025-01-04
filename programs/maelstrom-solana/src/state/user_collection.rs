use anchor_lang::prelude::*;

#[account]
pub struct UserCollection {
    pub collection_id: u8,
    pub gold: u16,
    // [16, 0...] means 1 count of cardid 0
    // bit rep 0001 0000
    // [1, 0...] means 1 count of cardid 1
    // bit rep 0000 0001
    pub card_count: [u8; 100],
    pub bump: u8,
}