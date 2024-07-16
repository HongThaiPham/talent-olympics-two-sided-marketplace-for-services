use anchor_lang::prelude::*;
use mpl_core::types::{Attribute, Attributes, Creator, DataState, PermanentFreezeDelegate, PermanentTransferDelegate, Plugin, PluginAuthority, PluginAuthorityPair, Royalties, RuleSet};

use crate::{error::MyErrorCode, Service, Vendor, DISCRIMINATOR_SIZE, SERVICE_SEED, VENDOR_SEED};

#[derive(Accounts)]
#[instruction(vendor_id: u64, id: u64)]
pub struct CreateService<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        seeds = [VENDOR_SEED.as_ref(), vendor_id.to_le_bytes().as_ref()],
        bump,
        constraint = vendor.authority == signer.key() @MyErrorCode::Unauthorized
      )]
    pub vendor: Account<'info, Vendor>,
    #[account(
        init,
        payer = signer,
        space = DISCRIMINATOR_SIZE + Service::INIT_SPACE,
        seeds = [SERVICE_SEED.as_ref(), vendor.key().as_ref() , id.to_le_bytes().as_ref()], 
        bump,
    )]
    pub service: Account<'info, Service>,
    /// The address of the new service asset
    #[account(mut)]
    pub asset: Signer<'info>,
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

impl<'info>  CreateService<'info> {
    pub fn handler(&mut self,_vendor_id: u64, id: u64, price:u64, name: String, uri: String, agreements: String, is_soulbound: bool, royalty_basis_points: u16 ) -> Result<()> {
       self.service.set_inner(
            Service {  
                id, 
                vendor: self.vendor.to_account_info().key(), 
                price, 
                name: name.clone(), 
                agreements: agreements.clone() ,
                asset: self.asset.to_account_info().key()
            }
       );
       self.mint_service_nft(name, uri, agreements, is_soulbound, royalty_basis_points)?;
        Ok(())
    }

    fn mint_service_nft(&self, name: String, uri: String, agreements: String, is_soulbound: bool, royalty_basis_points: u16)->Result<()> {
       

        let mut asset_plugins = vec![
            PluginAuthorityPair {
                plugin: Plugin::PermanentTransferDelegate( PermanentTransferDelegate{}),
                authority: Some(PluginAuthority::Address { address: self.service.to_account_info().key() }),
            },
            PluginAuthorityPair {
                plugin: Plugin::Royalties(Royalties {
                    basis_points: royalty_basis_points,
                    creators: vec![
                        Creator {
                            address: self.signer.to_account_info().key(),
                            percentage: 100
                        }
                    ],
                    rule_set: RuleSet::None
                }),
                authority: Some(PluginAuthority::Address { address: self.service.to_account_info().key() }),
            },
            PluginAuthorityPair {
                plugin: Plugin::Attributes(Attributes {
                    attribute_list: vec![
                        Attribute {
                            key: "name".to_string(),
                            value: name.clone()
                        },
                        Attribute {
                            key: "uri".to_string(),
                            value: uri.clone()
                        },
                        Attribute {
                            key: "agreements".to_string(),
                            value: agreements
                        }

                    ]
                }),
                authority: Some(PluginAuthority::Address { address: self.service.to_account_info().key() }),
            }
        ];

        // The Permanent Freeze Delegate plugin is a Permanent plugin that will always be present on the MPL Core Asset or MPL Core Collection to which it is added. A permanent plugin can only be added at the time of Asset or Collection creation.

        // The Permanent Freeze Plugin will work in areas such as:

        // Soulbound Tokens.

        if is_soulbound {
            asset_plugins.push(     
                PluginAuthorityPair {
                    plugin: Plugin::PermanentFreezeDelegate( PermanentFreezeDelegate { frozen: true }),
                    authority: Some(PluginAuthority::Address { address: self.service.to_account_info().key() })
                }
            );
        };

        mpl_core::instructions::CreateV1Cpi {
            asset: &self.asset.to_account_info(),
            collection: None,
            authority: Some(&self.signer.to_account_info()),
            payer: &self.signer.to_account_info(),
            owner: None,
            update_authority: Some(self.signer.as_ref()),
            system_program: &self.system_program.to_account_info(),
            log_wrapper: self.log_wrapper.as_ref(),
            __program: &self.mpl_core,
            __args: mpl_core::instructions::CreateV1InstructionArgs {
                data_state: DataState::AccountState,
                name,
                uri,
                plugins: Some(asset_plugins),
            },
        }
        .invoke()?;
        
        Ok(())
    }
}
