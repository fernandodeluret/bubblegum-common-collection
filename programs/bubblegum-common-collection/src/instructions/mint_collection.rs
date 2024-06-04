use crate::state::CollectionAuthority;
use anchor_lang::prelude::*;
use mpl_bubblegum::instructions::{MintV1Cpi, MintV1CpiAccounts, MintV1InstructionArgs};
use mpl_bubblegum::types::{MetadataArgs, TokenProgramVersion};

#[derive(Accounts)]
pub struct MintCollectionAccounts<'info> {
    #[account(
        seeds = [b"collection_authority"],
        bump = collection_authority.bump,
    )]
    pub collection_authority: Account<'info, CollectionAuthority>,
    /// CHECK: checked by bubblegum in CPI
    pub bubblegum_program: AccountInfo<'info>,
    /// CHECK: checked by bubblegum in CPI
    pub tree_config: AccountInfo<'info>,
    /// CHECK: checked by bubblegum in CPI
    pub leaf_owner: AccountInfo<'info>,
    /// CHECK: checked by bubblegum in CPI
    pub leaf_delegate: AccountInfo<'info>,
    /// CHECK: checked by bubblegum in CPI
    pub merkle_tree: AccountInfo<'info>,
    /// CHECK: checked by bubblegum in CPI
    pub payer: AccountInfo<'info>,
    /// CHECK: checked by bubblegum in CPI
    pub tree_creator_or_delegate: AccountInfo<'info>,
    /// CHECK: checked by bubblegum in CPI
    pub log_wrapper: AccountInfo<'info>,
    /// CHECK: checked by bubblegum in CPI
    pub compression_program: AccountInfo<'info>,
    /// CHECK: checked by bubblegum in CPI
    pub system_program: AccountInfo<'info>,
}

pub fn mint_collection_handler(ctx: Context<MintCollectionAccounts>) -> Result<()> {
    let metadata = MetadataArgs {
        name: "".to_string(),
        symbol: "".to_string(),
        uri: "".to_string(),
        seller_fee_basis_points: 0,
        primary_sale_happened: false,
        is_mutable: false,
        edition_nonce: None,
        token_standard: None,
        collection: None,
        uses: None,
        token_program_version: TokenProgramVersion::Original,
        creators: vec![],
    };

    let cpi_mint = MintV1Cpi::new(
        &ctx.accounts.bubblegum_program,
        MintV1CpiAccounts {
            tree_config: &ctx.accounts.tree_config,
            leaf_owner: &ctx.accounts.leaf_owner,
            leaf_delegate: &ctx.accounts.leaf_delegate,
            merkle_tree: &ctx.accounts.merkle_tree,
            payer: &ctx.accounts.payer,
            tree_creator_or_delegate: &ctx.accounts.tree_creator_or_delegate,
            log_wrapper: &ctx.accounts.log_wrapper,
            compression_program: &ctx.accounts.compression_program,
            system_program: &ctx.accounts.system_program,
        },
        MintV1InstructionArgs { metadata },
    );

    let signer_seeds: &[&[u8]] = &[&[0]];
    cpi_mint.invoke_signed(&[&signer_seeds])?;

    Ok(())
}
