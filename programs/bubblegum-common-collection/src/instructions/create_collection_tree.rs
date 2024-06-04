use crate::state::CollectionAuthority;
use anchor_lang::prelude::*;
use mpl_bubblegum::instructions::{
    CreateTreeConfigCpi, CreateTreeConfigCpiAccounts, CreateTreeConfigInstructionArgs,
};

#[derive(Accounts)]
pub struct CreateCollectionTreeAccounts<'info> {
    #[account(
        seeds = [b"collection_authority"],
        bump = collection_authority.bump,
    )]
    pub collection_authority: Account<'info, CollectionAuthority>,
    /// CHECK: checked by bubblegum in CPI
    pub bubblegum_program: AccountInfo<'info>,
    /// CHECK: checked by bubblegum in CPI
    #[account(mut)]
    pub tree_config: AccountInfo<'info>,
    /// CHECK: checked by bubblegum in CPI
    #[account(mut)]
    pub merkle_tree: AccountInfo<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: checked by bubblegum in CPI
    pub log_wrapper: AccountInfo<'info>,
    /// CHECK: checked by bubblegum in CPI
    pub compression_program: AccountInfo<'info>,
    /// CHECK: checked by bubblegum in CPI
    pub system_program: AccountInfo<'info>,
}

pub fn create_collection_tree_handler(
    ctx: Context<CreateCollectionTreeAccounts>,
    args: CreateTreeConfigInstructionArgsWrapper,
) -> anchor_lang::Result<()> {
    let tree_creator = &ctx.accounts.collection_authority.to_account_info();
    let payer = &ctx.accounts.payer.to_account_info();

    let create_tree = CreateTreeConfigCpi::new(
        &ctx.accounts.bubblegum_program,
        CreateTreeConfigCpiAccounts {
            tree_config: &ctx.accounts.tree_config,
            merkle_tree: &ctx.accounts.merkle_tree,
            payer,
            tree_creator,
            log_wrapper: &ctx.accounts.log_wrapper,
            compression_program: &ctx.accounts.compression_program,
            system_program: &ctx.accounts.system_program,
        },
        args.get_create_tree_config_instruction_args(),
    );

    let signer_seeds = [
        b"collection_authority".as_ref(),
        &[ctx.accounts.collection_authority.bump],
    ];
    create_tree.invoke_signed(&[&signer_seeds])?;

    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize, Eq, PartialEq, Clone, Debug)]
pub struct CreateTreeConfigInstructionArgsWrapper {
    pub max_depth: u32,
    pub max_buffer_size: u32,
    pub public: Option<bool>,
}
impl CreateTreeConfigInstructionArgsWrapper {
    pub fn get_create_tree_config_instruction_args(&self) -> CreateTreeConfigInstructionArgs {
        CreateTreeConfigInstructionArgs {
            max_depth: self.max_depth,
            max_buffer_size: self.max_buffer_size,
            public: self.public,
        }
    }
}
