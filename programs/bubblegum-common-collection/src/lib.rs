pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("4hE1H9kAU1S28ZL4x9264G9SUFEGvEVt2yv1BDChZ8fa");

#[program]
pub mod bubblegum_common_collection {
    use super::*;

    pub fn initialize_collection_authority(
        ctx: Context<InitializeCollectionAuthorityAccounts>,
    ) -> Result<()> {
        initialize_collection_authority_handler(ctx)
    }

    pub fn mint_collection(ctx: Context<MintCollectionAccounts>) -> Result<()> {
        mint_collection_handler(ctx)
    }

    pub fn create_collection_tree(
        ctx: Context<CreateCollectionTreeAccounts>,
        args: CreateTreeConfigInstructionArgsWrapper,
    ) -> Result<()> {
        create_collection_tree_handler(ctx, args)
    }
}
