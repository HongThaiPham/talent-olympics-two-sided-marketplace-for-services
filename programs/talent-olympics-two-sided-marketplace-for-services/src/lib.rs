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

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }
}
