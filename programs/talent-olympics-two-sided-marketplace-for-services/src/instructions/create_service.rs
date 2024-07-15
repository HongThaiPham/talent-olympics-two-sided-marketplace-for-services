use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateService<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}
