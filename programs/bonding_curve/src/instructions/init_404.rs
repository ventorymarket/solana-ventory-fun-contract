use anchor_lang::prelude::*;
use crate::errors::CustomError;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount, mint_to, MintTo},
    metadata::Metadata
};
use anchor_spl::metadata::mpl_token_metadata::{
    instructions::{
        CreateMasterEditionV3Cpi, 
        CreateMasterEditionV3CpiAccounts, 
        CreateMasterEditionV3InstructionArgs, 
        CreateMetadataAccountV3Cpi, 
        CreateMetadataAccountV3CpiAccounts, 
        CreateMetadataAccountV3InstructionArgs
    }, 
    types::{
        CollectionDetails,
        Creator, 
        DataV2
    }
};
use crate::state::*;

pub fn init_token404(
    ctx: Context<InitToken404>,
    token_name: String,
    token_symbol: String,
    token_uri: String,
    base_uri: String,
    uri_size: u16,
    path: Vec<u16>,
) -> Result<()> {
    // Create liquidity pool
    let pool = &mut ctx.accounts.pool;

    let sum : u16 = path.iter().sum();
    msg!("Path sum is valid: {:?} {}", path,sum);
    require!(sum == 1000, CustomError::InvalidSum);
    require!(path.len() as u16 == uri_size, CustomError::EmptyArrayUri);

    if pool.type_c != 0{
        return err!(CustomError::PendingInit);
    }
    
    msg!("Create liquidity pool");
    let collection_mint_key = ctx.accounts.collection_mint.key();

    let collection_seeds = &[
        &b"authority"[..],
        collection_mint_key.as_ref(),
        &[ctx.bumps.mint_authority]
    ];
    let signer_seeds = &[&collection_seeds[..]];
    msg!("signer_seeds");

    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_accounts = MintTo {
        mint: ctx.accounts.collection_mint.to_account_info(),
        to: ctx.accounts.destination.to_account_info(),
        authority: ctx.accounts.mint_authority.to_account_info(),
    };
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
    mint_to(cpi_ctx, 1)?;
    msg!("Collection NFT minted!");
    
    let creator = vec![
        Creator {
            address: ctx.accounts.mint_authority.key().clone(),
            verified: true,
            share: 100,
        },
    ];
    
    let metadata = &ctx.accounts.metadata.to_account_info();
    let master_edition = &ctx.accounts.master_edition.to_account_info();
    let mint = &ctx.accounts.collection_mint.to_account_info();
    let authority = &ctx.accounts.mint_authority.to_account_info();
    let payer = &ctx.accounts.payer.to_account_info();
    let system_program = &ctx.accounts.system_program.to_account_info();
    let spl_token_program = &ctx.accounts.token_program.to_account_info();
    let spl_metadata_program = &ctx.accounts.token_metadata_program.to_account_info();

    let metadata_account = CreateMetadataAccountV3Cpi::new(
        spl_metadata_program, 
        CreateMetadataAccountV3CpiAccounts {
            metadata,
            mint,
            mint_authority: authority,
            payer,
            update_authority: (authority, true),
            system_program,
            rent: None,
        },
        CreateMetadataAccountV3InstructionArgs {
            data: DataV2 {
                name: token_name.to_owned(),
                symbol: token_symbol.to_owned(),
                uri: token_uri.to_owned(),
                seller_fee_basis_points: 0,
                creators: Some(creator),
                collection: None,
                uses: None,
            },
            is_mutable: true,
            collection_details: Some(
                CollectionDetails::V1 { 
                    size: 0 
                }
            )
        }
    );
    metadata_account.invoke_signed(signer_seeds)?;
    msg!("Metadata Account created!");

    let master_edition_account = CreateMasterEditionV3Cpi::new(
        spl_metadata_program,
        CreateMasterEditionV3CpiAccounts {
            edition: master_edition,
            update_authority: authority,
            mint_authority: authority,
            mint,
            payer,
            metadata,
            token_program: spl_token_program,
            system_program,
            rent: None,
        },
        CreateMasterEditionV3InstructionArgs {
            max_supply: Some(0),
        }
    );
    master_edition_account.invoke_signed(signer_seeds)?;
    msg!("Master Edition Account created");
    let initial_supply = 1_000_000_000 * 10u64.pow(9 as u32);
    pool.type_c = 2;

    let pool_escrow = &mut ctx.accounts.pool_escrow;
    pool_escrow.set_inner(EscrowPool::new(
        ctx.accounts.collection_mint.key().clone(),
        pool.token.key().clone(),
        token_name.clone(),
        token_symbol.clone(),
        base_uri.clone(),
        uri_size.clone(),
        path.clone(),
        ctx.bumps.pool_escrow
    ));

    emit!(CreatePool404{
        token_address: pool.token,
        collection_address: ctx.accounts.collection_mint.key(),
        pool_escrow: pool_escrow.key(),
        creater:  ctx.accounts.payer.key(),
        token_name,
        token_symbol,
        initial_supply,
        token_uri,
        base_uri,
        uri_size,
        path
    });
    Ok(())
}

#[derive(Accounts)]
// #[instruction(
//     token_name: String,
//     token_symbol: String,
//     token_uri: String,
//     base_uri: String,
//     uri_size: u16,
//     path: Vec<u8>
// )]
pub struct InitToken404<'info> {
    
    #[account(
        init,
        space = EscrowPool::get_space(),
        payer = payer,
        seeds = [EscrowPool::POOL_SEED_PREFIX.as_bytes(), pool.token.key().as_ref(), collection_mint.key().as_ref()],
        bump
    )]
    pub pool_escrow: Box<Account<'info, EscrowPool>>,

    #[account(
        init,
        payer = payer,
        mint::decimals = 0,
        mint::authority = mint_authority,
        mint::freeze_authority = mint_authority,
    )]
    pub collection_mint: Box<Account<'info, Mint>>,

    /// CHECK: This account is not initialized and is being used for signing purposes only
    #[account(
        seeds = [b"authority", collection_mint.key().as_ref()],
        bump,
    )]
    pub mint_authority: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: This account will be initialized by the metaplex program
    pub metadata: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: This account will be initialized by the metaplex program
    pub master_edition: UncheckedAccount<'info>,

    #[account(
        init,
        payer = payer,
        associated_token::mint = collection_mint,
        associated_token::authority = pool
    )]
    pub destination: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [LiquidityPool::POOL_SEED_PREFIX.as_bytes(), pool.token.key().as_ref()],
        bump = pool.bump
    )]
    pub pool: Box<Account<'info, LiquidityPool>>,
   
    #[account(mut)]
    pub payer: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[event]
pub struct CreatePool404 {
    pub token_address: Pubkey,
    pub collection_address: Pubkey,
    pub pool_escrow: Pubkey,
    pub creater: Pubkey,
    pub token_name: String,
    pub token_symbol: String,
    pub token_uri: String,
    pub initial_supply: u64,
    pub base_uri: String,
    pub uri_size: u16,
    pub path: Vec<u16>,
}