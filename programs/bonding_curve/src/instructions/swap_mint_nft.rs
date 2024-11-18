use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{Metadata, MetadataAccount, MasterEditionAccount},
    token::{mint_to, Mint, Token, TokenAccount},
};
use anchor_spl::metadata::mpl_token_metadata::{
    instructions::{
        CreateMasterEditionV3Cpi, CreateMetadataAccountV3Cpi, VerifyCollectionV1Cpi,
        CreateMetadataAccountV3CpiAccounts, CreateMasterEditionV3CpiAccounts,
        CreateMetadataAccountV3InstructionArgs, CreateMasterEditionV3InstructionArgs,
        VerifyCollectionV1CpiAccounts
    },
    types::{Collection, Creator, DataV2},
};
use crate::{state::*, errors::CustomError};

const TOKEN_TRANSFER_AMOUNT: u64 = 1_000_000 * 1_000_000_000;

#[derive(Accounts)]
pub struct MintAndVerifyNFT<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(
        mut,
        seeds = [
            EscrowPool::POOL_SEED_PREFIX.as_bytes(),
            token_mint.key().as_ref(),
            collection_mint.key().as_ref()
        ],
        bump
    )]
    pub pool_escrow: Box<Account<'info, EscrowPool>>,

    #[account(mut)]
    pub token_mint: Box<Account<'info, Mint>>,

    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = user,
    )]
    pub user_token_account: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = pool_escrow,
    )]
    pub pool_escrow_token_account: Box<Account<'info, TokenAccount>>,

    #[account(
        init,
        payer = user,
        mint::decimals = 0,
        mint::authority = mint_authority,
        mint::freeze_authority = mint_authority,
    )]
    pub mint_nft: Box<Account<'info, Mint>>,
    
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_nft,
        associated_token::authority = user
    )]
    pub destination: Box<Account<'info, TokenAccount>>,
    
    /// CHECK: Metadata PDA 
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    
    /// CHECK: Master Edition PDA
    #[account(mut)]
    pub master_edition: UncheckedAccount<'info>,
    
    /// CHECK: Authority PDA for signing
    #[account(
        seeds = [b"authority", collection_mint.key().as_ref()],
        bump,
    )]
    pub mint_authority: UncheckedAccount<'info>,
    
    #[account(mut)]
    pub collection_mint: Box<Account<'info, Mint>>,
    
    #[account(mut)]
    pub collection_metadata: Box<Account<'info, MetadataAccount>>,
    
    pub collection_master_edition: Box<Account<'info, MasterEditionAccount>>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_metadata_program: Program<'info, Metadata>,
    
    /// CHECK: Sysvar instruction account
    #[account(address = anchor_lang::solana_program::sysvar::instructions::ID)]
    pub sysvar_instruction: UncheckedAccount<'info>,
}

fn get_number_at_position(
    mut arr: Vec<u16>,
    mut min: u16,
    mut max: u16
) -> (u16, Vec<u16>, u16, u16) {
    let narr = arr.len() as u16;
    if narr == 1 {
        return (1, arr, min, max);
    }

    let idtoken = if arr[max as usize] > 0 {
        arr[max as usize] -= 1;
        max += 1;
        max
    } else {
        1
    };

    min = if arr[min as usize] == 0 { min + 1 } else { min };
    max = if max >= narr { min } else { max };
    
    (idtoken, arr, min, max)
}

pub fn mint_and_verify(ctx: Context<MintAndVerifyNFT>) -> Result<()> {
    let pool_escrow = &mut ctx.accounts.pool_escrow;
    require!(pool_escrow.count < 1000, CustomError::LimitMintNft);

    // Setup signing authority
    let collection_mint_key = pool_escrow.collection.key();
    let seeds = &[
        &b"authority"[..],
        collection_mint_key.as_ref(),
        &[ctx.bumps.mint_authority]
    ];
    let signer_seeds = &[&seeds[..]];

    // Mint NFT token
    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::MintTo {
                mint: ctx.accounts.mint_nft.to_account_info(),
                to: ctx.accounts.destination.to_account_info(),
                authority: ctx.accounts.mint_authority.to_account_info(),
            },
            signer_seeds
        ),
        1
    )?;

    // Setup metadata content
    let token_id = pool_escrow.count + 1;
    let (number, updated_path, updated_min, updated_max) = get_number_at_position(
        pool_escrow.path.clone(),
        pool_escrow.path_min,
        pool_escrow.path_max
    );
    
    pool_escrow.path = updated_path;
    pool_escrow.path_min = updated_min;
    pool_escrow.path_max = updated_max;

    // Create metadata
    CreateMetadataAccountV3Cpi::new(
        &ctx.accounts.token_metadata_program.to_account_info(),
        CreateMetadataAccountV3CpiAccounts {
            metadata: &ctx.accounts.metadata.to_account_info(),
            mint: &ctx.accounts.mint_nft.to_account_info(),
            mint_authority: &ctx.accounts.mint_authority.to_account_info(),
            payer: &ctx.accounts.user.to_account_info(),
            update_authority: (&ctx.accounts.mint_authority.to_account_info(), true),
            system_program: &ctx.accounts.system_program.to_account_info(),
            rent: None,
        },
        CreateMetadataAccountV3InstructionArgs {
            data: DataV2 {
                name: format!("{} #{}", pool_escrow.name, token_id),
                symbol: pool_escrow.symbol.clone(),
                uri: format!("{}{}.json", pool_escrow.base_uri, number),
                seller_fee_basis_points: 0,
                creators: Some(vec![Creator {
                    address: ctx.accounts.mint_authority.key(),
                    verified: true,
                    share: 100,
                }]),
                collection: Some(Collection {
                    verified: false,
                    key: pool_escrow.collection.key(),
                }),
                uses: None
            },
            is_mutable: true,
            collection_details: None,
        }
    ).invoke_signed(signer_seeds)?;

    // Create master edition
    CreateMasterEditionV3Cpi::new(
        &ctx.accounts.token_metadata_program.to_account_info(),
        CreateMasterEditionV3CpiAccounts {
            edition: &ctx.accounts.master_edition.to_account_info(),
            mint: &ctx.accounts.mint_nft.to_account_info(),
            update_authority: &ctx.accounts.mint_authority.to_account_info(),
            mint_authority: &ctx.accounts.mint_authority.to_account_info(),
            payer: &ctx.accounts.user.to_account_info(),
            metadata: &ctx.accounts.metadata.to_account_info(),
            token_program: &ctx.accounts.token_program.to_account_info(),
            system_program: &ctx.accounts.system_program.to_account_info(),
            rent: None,
        },
        CreateMasterEditionV3InstructionArgs {
            max_supply: Some(0),
        }
    ).invoke_signed(signer_seeds)?;

    // Verify collection
    VerifyCollectionV1Cpi::new(
        &ctx.accounts.token_metadata_program.to_account_info(),
        VerifyCollectionV1CpiAccounts {
            authority: &ctx.accounts.mint_authority.to_account_info(),
            delegate_record: None,
            metadata: &ctx.accounts.metadata.to_account_info(),
            collection_mint: &ctx.accounts.collection_mint.to_account_info(),
            collection_metadata: Some(&ctx.accounts.collection_metadata.to_account_info()),
            collection_master_edition: Some(&ctx.accounts.collection_master_edition.to_account_info()),
            system_program: &ctx.accounts.system_program.to_account_info(),
            sysvar_instructions: &ctx.accounts.sysvar_instruction.to_account_info(),
        }
    ).invoke_signed(signer_seeds)?;

    // Transfer tokens
    anchor_spl::token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::Transfer {
                from: ctx.accounts.user_token_account.to_account_info(),
                to: ctx.accounts.pool_escrow_token_account.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            }
        ),
        TOKEN_TRANSFER_AMOUNT
    )?;
    
    // Update pool state
    pool_escrow.amount += TOKEN_TRANSFER_AMOUNT;
    pool_escrow.count += 1;

    // Emit event
    emit_transfer_event(
        pool_escrow.collection.key(),
        pool_escrow.token.key(),
        pool_escrow.key(),
        pool_escrow.key(),
        ctx.accounts.user.key(),
        ctx.accounts.mint_nft.key()
    );

    Ok(())
}