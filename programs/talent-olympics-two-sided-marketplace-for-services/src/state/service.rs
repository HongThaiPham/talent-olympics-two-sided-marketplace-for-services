use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Service {
    pub id: u64,
    pub vendor: Pubkey,
    pub asset: Pubkey,
    pub price: u64,
    #[max_len(20)]
    pub name: String,
    #[max_len(500)]
    pub agreements: String,
}
