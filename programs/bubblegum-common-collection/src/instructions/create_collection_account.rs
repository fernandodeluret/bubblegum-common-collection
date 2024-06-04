use anchor_lang::prelude::*;
use crate::state::CollectionAuthority;
use mpl_token_metadata::instructions::{CreateV1Cpi, CreateV1CpiAccounts, CreateV1InstructionArgs, MintV1Cpi, MintV1CpiAccounts, MintV1InstructionArgs};
use mpl_token_metadata::types::{TokenStandard};

#[derive(Accounts)]
pub struct CreateCollectionAccountAccounts<'info> {
    #[account(
        seeds = [b"collection_authority"],
        bump = collection_authority.bump,
    )]
    pub collection_authority: Account<'info, CollectionAuthority>,
    /// CHECK: checked by bubblegum in CPI
    #[account(mut)]
    pub metadata: AccountInfo<'info>,
    /// CHECK: checked by bubblegum in CPI
    #[account(mut)]
    pub master_edition: AccountInfo<'info>,
    /// CHECK: checked by bubblegum in CPI
    #[account(mut)]
    pub mint: Signer<'info>,
    pub authority: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: checked by bubblegum in CPI
    pub system_program: AccountInfo<'info>,
    /// CHECK: checked by bubblegum in CPI
    pub sysvar_instructions: AccountInfo<'info>,
    /// CHECK: checked by bubblegum in CPI
    pub spl_token_program: AccountInfo<'info>,
    /// CHECK: checked by bubblegum in CPI
    pub mpl_token_metadata_program: AccountInfo<'info>,
    /// CHECK: checked by bubblegum in CPI
    #[account(mut)]
    pub token: AccountInfo<'info>,
    /// CHECK: checked by bubblegum in CPI
    pub token_owner: AccountInfo<'info>,
    /// CHECK: checked by bubblegum in CPI
    pub spl_ata_program: AccountInfo<'info>,
}

pub fn create_collection_account_handler(
    ctx: Context<CreateCollectionAccountAccounts>,
    // args: CreateInstructionArgs,
) -> Result<()> {
    let update_authority = &ctx.accounts.collection_authority.to_account_info();
    let authority = &ctx.accounts.authority.to_account_info();
    let payer = &ctx.accounts.payer.to_account_info();
    let signer_seeds = [
        b"collection_authority".as_ref(),
        &[ctx.accounts.collection_authority.bump],
    ];

    let create_cpi = CreateV1Cpi::new(
        &ctx.accounts.mpl_token_metadata_program,
        CreateV1CpiAccounts {
            metadata: &ctx.accounts.metadata,
            // validates the authority:
            // - NonFungible must have a "valid" master edition
            // - Fungible must have the authority as the mint_authority
            master_edition: Some(&ctx.accounts.master_edition),
            mint: (&ctx.accounts.mint, false),  //no docs at all for this boolean...
            authority,
            payer,
            update_authority: (update_authority, true), //we are passing `collection_authority` here, but it could be any PDA from the program
            system_program: &ctx.accounts.system_program,
            sysvar_instructions: &ctx.accounts.sysvar_instructions,
            spl_token_program: Some(&ctx.accounts.spl_token_program),
        },
        CreateV1InstructionArgs {
            name: "".to_string(),
            symbol: "".to_string(),
            uri: "".to_string(),
            seller_fee_basis_points: 0,
            creators: None,
            primary_sale_happened: false,
            is_mutable: false,
            token_standard: TokenStandard::NonFungible,
            collection: None,
            uses: None,
            collection_details: None,
            rule_set: None,
            decimals: None,
            print_supply: None,
        },
    );

    create_cpi.invoke_signed(&[&signer_seeds])?;

    let mint_cpi = MintV1Cpi::new(&ctx.accounts.mpl_token_metadata_program, MintV1CpiAccounts {
        token: &ctx.accounts.token,
        token_owner: Some(&ctx.accounts.token_owner),
        metadata: &ctx.accounts.metadata,
        master_edition: Some(&ctx.accounts.master_edition),
        token_record: None,  //For TokenStandard.ProgrammableNonFungible
        mint: &ctx.accounts.mint,
        authority, // /** (Mint or Update) authority */
        delegate_record: None,
        payer,
        system_program: &ctx.accounts.system_program,
        sysvar_instructions: &ctx.accounts.sysvar_instructions,
        spl_token_program: &ctx.accounts.spl_token_program,
        spl_ata_program: &ctx.accounts.spl_ata_program,
        authorization_rules_program: None,
        authorization_rules: None,
    }, MintV1InstructionArgs { amount: 1, authorization_data: None });

    mint_cpi.invoke_signed(&[&signer_seeds])?;

    Ok(())
}
