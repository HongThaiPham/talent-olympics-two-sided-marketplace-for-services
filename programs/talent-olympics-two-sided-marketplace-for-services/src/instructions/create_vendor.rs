use anchor_lang::prelude::*;

use crate::{Vendor, DISCRIMINATOR_SIZE, VENDOR_SEED};

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct CreateVendor<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
      init,
      payer = signer,
      seeds = [VENDOR_SEED.as_ref(), id.to_le_bytes().as_ref()],
      bump,
      space = DISCRIMINATOR_SIZE + Vendor::INIT_SPACE,
    )]
    pub vendor: Account<'info, Vendor>,

    pub system_program: Program<'info, System>,
}

impl<'info> CreateVendor<'info> {
    pub fn handler(&mut self, id: u64, name: String) -> Result<()> {
        self.vendor.set_inner(Vendor {
            vendor_id: id,
            authority: self.signer.to_account_info().key(),
            name,
        });

        Ok(())
    }
}
