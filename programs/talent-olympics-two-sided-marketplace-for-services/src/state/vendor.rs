use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Vendor {
    pub id: u64,
    pub authority: Pubkey,
    #[max_len(20)]
    pub name: String,
}
