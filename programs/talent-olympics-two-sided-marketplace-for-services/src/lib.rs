pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("3uTTtinfRPsvpfszHMX2MDv5BEihCXHTwGUssWAPTzme");

#[program]
pub mod talent_olympics_two_sided_marketplace_for_services {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, fee: u64) -> Result<()> {
        ctx.accounts.handler(fee)
    }

    pub fn set_fee(ctx: Context<SetFee>, fee: u64) -> Result<()> {
        ctx.accounts.handler(fee)
    }

    pub fn create_vendor(ctx: Context<CreateVendor>, id: u64, name: String) -> Result<()> {
        ctx.accounts.handler(id, name)
    }

    pub fn create_service(
        ctx: Context<CreateService>,
        vendor_id: u64,
        id: u64,
        price: u64,
        name: String,
        uri: String,
        agreements: String,
        is_soulbound: bool,
        royalty_basis_points: u16,
    ) -> Result<()> {
        ctx.accounts.handler(
            vendor_id,
            id,
            price,
            name,
            uri,
            agreements,
            is_soulbound,
            royalty_basis_points,
        )
    }

    pub fn purchase(ctx: Context<Purchase>, id: u64) -> Result<()> {
        ctx.accounts.handler(id, ctx.bumps)
    }
}
