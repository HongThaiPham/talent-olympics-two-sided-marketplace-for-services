

use anchor_lang::{prelude::*, system_program::{self, Transfer}};

use crate::{error::MyErrorCode, ProtocolConfig, Service, Vendor, CONFIG_SEED, SERVICE_SEED, VAULT_SEED};

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct Purchase<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    /// CHECK: The address of this service asset
    #[account(mut)]
    pub vendor_authority: SystemAccount<'info>,
    #[account(
      seeds = [CONFIG_SEED.as_ref()],
      bump,
      has_one = vault
    )]
    pub config: Account<'info, ProtocolConfig>,
    #[account(
      constraint = vendor.authority == vendor_authority.key() @MyErrorCode::Unauthorized
    )]
    pub vendor: Account<'info, Vendor>,
    #[account(
      seeds = [SERVICE_SEED.as_ref(), vendor.key().as_ref() , id.to_le_bytes().as_ref()], 
      bump,
      has_one = vendor,
      has_one = asset,
    )]
    pub service: Account<'info, Service>,
    /// CHECK: it's ok to use
    #[account(
        mut,
        seeds = [VAULT_SEED.as_ref()],
        bump
    )]
    pub vault: AccountInfo<'info>,
    /// The address of the asset.
    /// CHECK: Checked in mpl-core.
    #[account(mut)]
    pub asset: AccountInfo<'info>,
    /// The SPL Noop program.
    /// CHECK: Checked in mpl-core.
    pub log_wrapper: Option<AccountInfo<'info>>,
    
    /// The MPL Core program.
    /// CHECK: Checked in mpl-core.
    #[account(address = mpl_core::ID)]
    pub mpl_core: AccountInfo<'info>,
    
    // The system program.
    pub system_program: Program<'info, System>,
}

impl<'info> Purchase<'info> {
    pub fn handler(&mut self, id: u64, bumps: PurchaseBumps) -> Result<()> {

      let asset_price = self.service.price;
      self.transfer_sol_to_vendor(asset_price)?;
      self.collect_fee()?;
      self.transfer_asset_to_buyer(id, bumps.service)?;
      Ok(())
    }


    fn collect_fee(&self) -> Result<()> {
        let cpi_accounts = Transfer {
            from: self.signer.to_account_info(),
            to: self.vault.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(self.system_program.to_account_info(), cpi_accounts);
        system_program::transfer(cpi_ctx, self.config.fee)?;
        Ok(())
    }

    fn transfer_asset_to_buyer(&self, id:u64, service_bump: u8) -> Result<()> {
      let vendor_key = self.vendor.to_account_info().key();
      let service_id = id.to_le_bytes();
      let signer_seeds = &[
            SERVICE_SEED,
            vendor_key.as_ref(),
            service_id.as_ref(),
            &[service_bump],
        ];
      mpl_core::instructions::TransferV1Cpi {
        asset: &self.asset.to_account_info(),
        collection: None,
        payer: &self.signer.to_account_info(),
        authority: Some(&self.service.to_account_info()),
        new_owner: &self.signer.to_account_info(),
        system_program: Some(&self.system_program.to_account_info()),
        log_wrapper: self.log_wrapper.as_ref(),
        __program: &self.mpl_core,
        __args: mpl_core::instructions::TransferV1InstructionArgs {
            compression_proof: None,
        },
      }.invoke_signed(&[signer_seeds])?;
      Ok(())
    }

    fn transfer_sol_to_vendor(&self, amount: u64) -> Result<()> {
        let cpi_accounts = Transfer {
            from: self.signer.to_account_info(),
            to: self.vendor_authority.to_account_info(),
           
        };
        let cpi_ctx = CpiContext::new(self.system_program.to_account_info(), cpi_accounts);
        system_program::transfer(cpi_ctx, amount)?;
        Ok(())
    }
}