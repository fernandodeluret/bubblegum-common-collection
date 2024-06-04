use crate::state::CollectionAuthority;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeCollectionAuthorityAccounts<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        seeds = [b"collection_authority"],
        bump,
        space = 8 + 1,
    )]
    pub collection_authority: Account<'info, CollectionAuthority>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_collection_authority_handler(
    ctx: Context<InitializeCollectionAuthorityAccounts>,
) -> Result<()> {
    ctx.accounts.collection_authority.bump = ctx.bumps.collection_authority;

    Ok(())
}
