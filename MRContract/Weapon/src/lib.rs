use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        create_master_edition_v3, create_metadata_accounts_v3,
        set_and_verify_sized_collection_item, sign_metadata, CreateMasterEditionV3,
        CreateMetadataAccountsV3, Metadata, SetAndVerifySizedCollectionItem, SignMetadata,
    },
    token::{self, Mint, MintTo, Token, TokenAccount, Transfer},
};

use mpl_token_metadata::{
    pda::{find_master_edition_account, find_metadata_account},
    state::{CollectionDetails, Creator, DataV2},
};
use solana_program::pubkey;
use solana_program::{pubkey::Pubkey, system_instruction};

declare_id!("3yQACoaLTgp85X7toZZCYE1qhDuHmREMauHNrkJEpEmJ");

#[constant]
const ADMIN_PUBKEY: Pubkey = pubkey!("7GzPPzAZn92X4VeVyNg36QZqJ25j16G6efc7wUzTKjph");

#[program]
pub mod mr_solana_weapon {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let seeds = b"collection";
        let bump = *ctx.bumps.get("collection_mint").unwrap();
        let signer: &[&[&[u8]]] = &[&[seeds, &[bump]]];

        token::mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    mint: ctx.accounts.collection_mint.to_account_info(),
                    authority: ctx.accounts.collection_mint.to_account_info(),
                    to: ctx.accounts.token_account.to_account_info(),
                },
                signer,
            ),
            1,
        )?;

        create_metadata_accounts_v3(
            CpiContext::new_with_signer(
                ctx.accounts.token_metadata_program.to_account_info(),
                CreateMetadataAccountsV3 {
                    metadata: ctx.accounts.metadata_account.to_account_info(),
                    mint: ctx.accounts.collection_mint.to_account_info(),
                    mint_authority: ctx.accounts.collection_mint.to_account_info(), // use pda mint address as mint authority
                    update_authority: ctx.accounts.collection_mint.to_account_info(), // use pda mint as update authority
                    payer: ctx.accounts.signer.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
                signer,
            ),
            DataV2 {
                name: "MR Weapon".to_string(),
                symbol: "MRW".to_string(),
                seller_fee_basis_points: 0,
                uri: "https://kgcdn.shenmezhideke.com/collection.json".to_string(),
                creators: Some(vec![Creator {
                    address: ctx.accounts.signer.key(),
                    verified: false,
                    share: 100,
                }]),
                collection: None,
                uses: None,
            },
            true,
            true,
            Some(CollectionDetails::V1 { size: 0 }),
        )?;

        create_master_edition_v3(
            CpiContext::new_with_signer(
                ctx.accounts.token_metadata_program.to_account_info(),
                CreateMasterEditionV3 {
                    payer: ctx.accounts.signer.to_account_info(),
                    mint: ctx.accounts.collection_mint.to_account_info(),
                    edition: ctx.accounts.master_edition.to_account_info(),
                    mint_authority: ctx.accounts.collection_mint.to_account_info(),
                    update_authority: ctx.accounts.collection_mint.to_account_info(),
                    metadata: ctx.accounts.metadata_account.to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
                signer,
            ),
            Some(0),
        )?;

        sign_metadata(CpiContext::new(
            ctx.accounts.token_metadata_program.to_account_info(),
            SignMetadata {
                creator: ctx.accounts.signer.to_account_info(),
                metadata: ctx.accounts.metadata_account.to_account_info(),
            },
        ))?;

        Ok(())
    }

    pub fn mint(ctx: Context<MintWeapon>) -> Result<()> {
        let seeds = b"collection";
        let bump = *ctx.bumps.get("collection_mint").unwrap();
        let signer: &[&[&[u8]]] = &[&[seeds, &[bump]]];

        let transfer_instruction =
            system_instruction::transfer(ctx.accounts.signer.key, &ADMIN_PUBKEY, 1_000_000_000);
        anchor_lang::solana_program::program::invoke_signed(
            &transfer_instruction,
            &[
                ctx.accounts.signer.to_account_info(),
                ctx.accounts.to.clone(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[],
        )?;

        token::mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    mint: ctx.accounts.token_mint.to_account_info(),
                    to: ctx.accounts.token_account.to_account_info(),
                    authority: ctx.accounts.collection_mint.to_account_info(),
                },
                signer,
            ),
            1,
        )?;

        create_metadata_accounts_v3(
            CpiContext::new_with_signer(
                ctx.accounts.token_metadata_program.to_account_info(),
                CreateMetadataAccountsV3 {
                    metadata: ctx.accounts.metadata_account.to_account_info(),
                    mint: ctx.accounts.token_mint.to_account_info(),
                    mint_authority: ctx.accounts.collection_mint.to_account_info(),
                    update_authority: ctx.accounts.collection_mint.to_account_info(),
                    payer: ctx.accounts.signer.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
                &signer,
            ),
            DataV2 {
                name: format!("Weapon T1"),
                symbol: "MRW".to_string(),
                uri: format!(
                    "https://kgcdn.shenmezhideke.com/{}.json",
                    ctx.accounts.token_mint.key().to_string()
                ),
                seller_fee_basis_points: 0,
                creators: Some(vec![Creator {
                    address: ctx.accounts.signer.key(),
                    verified: false,
                    share: 100,
                }]),
                collection: None,
                uses: None,
            },
            true,
            true,
            None,
        )?;

        create_master_edition_v3(
            CpiContext::new_with_signer(
                ctx.accounts.token_metadata_program.to_account_info(),
                CreateMasterEditionV3 {
                    payer: ctx.accounts.signer.to_account_info(),
                    mint: ctx.accounts.token_mint.to_account_info(),
                    edition: ctx.accounts.master_edition.to_account_info(),
                    mint_authority: ctx.accounts.collection_mint.to_account_info(),
                    update_authority: ctx.accounts.collection_mint.to_account_info(),
                    metadata: ctx.accounts.metadata_account.to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
                signer,
            ),
            Some(0),
        )?;

        set_and_verify_sized_collection_item(
            CpiContext::new_with_signer(
                ctx.accounts.token_metadata_program.to_account_info(),
                SetAndVerifySizedCollectionItem {
                    metadata: ctx.accounts.metadata_account.to_account_info(),
                    collection_authority: ctx.accounts.collection_mint.to_account_info(),
                    payer: ctx.accounts.signer.to_account_info(),
                    update_authority: ctx.accounts.collection_mint.to_account_info(),
                    collection_mint: ctx.accounts.collection_mint.to_account_info(),
                    collection_metadata: ctx.accounts.collection_metadata_account.to_account_info(),
                    collection_master_edition: ctx
                        .accounts
                        .collection_master_edition
                        .to_account_info(),
                },
                signer,
            ),
            None,
        )?;

        sign_metadata(CpiContext::new(
            ctx.accounts.token_metadata_program.to_account_info(),
            SignMetadata {
                creator: ctx.accounts.signer.to_account_info(),
                metadata: ctx.accounts.metadata_account.to_account_info(),
            },
        ))?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintWeapon<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(mut, address = ADMIN_PUBKEY)]
    pub to: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [b"collection"],
        bump,
    )]
    pub collection_mint: Account<'info, Mint>,

    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(
        mut,
        address = find_master_edition_account(& collection_mint.key()).0
    )]
    pub collection_master_edition: UncheckedAccount<'info>,

    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(
        mut,
        address = find_metadata_account(& collection_mint.key()).0
    )]
    pub collection_metadata_account: UncheckedAccount<'info>,

    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(
        init,
        payer = signer,
        mint::decimals = 0,
        mint::authority = collection_mint,
        mint::freeze_authority = collection_mint
    )]
    pub token_mint: Account<'info, Mint>,

    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(
        init,
        payer = signer,
        associated_token::mint = token_mint,
        associated_token::authority = signer
    )]
    pub token_account: Account<'info, TokenAccount>,

    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(
        mut,
        address = find_master_edition_account(& token_mint.key()).0
    )]
    pub master_edition: UncheckedAccount<'info>,

    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(
        mut,
        address = find_metadata_account(& token_mint.key()).0
    )]
    pub metadata_account: UncheckedAccount<'info>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        seeds = [b"collection"],
        bump,
        payer = signer,
        mint::decimals = 0,
        mint::authority = collection_mint,
        mint::freeze_authority = collection_mint
    )]
    pub collection_mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = collection_mint,
        associated_token::authority = signer
    )]
    pub token_account: Account<'info, TokenAccount>,

    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(
        mut,
        address = find_master_edition_account(& collection_mint.key()).0
    )]
    pub master_edition: UncheckedAccount<'info>,

    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(
        mut,
        address = find_metadata_account(& collection_mint.key()).0
    )]
    pub metadata_account: UncheckedAccount<'info>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}
