use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Vendor {
    pub vendor_id: u64,
    pub authority: Pubkey,
    #[max_len(20)]
    pub name: String,
}
