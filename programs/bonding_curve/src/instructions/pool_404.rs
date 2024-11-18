use anchor_lang::prelude::*;
use anchor_lang::system_program::{self};

use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount, mint_to, set_authority},
    metadata::{create_metadata_accounts_v3, CreateMetadataAccountsV3, Metadata},
};
use anchor_spl::metadata::mpl_token_metadata::types::DataV2;
use crate::{consts::INITIAL_LAMPORTS_FOR_POOL, state::*};

#[derive(Accounts)]
pub struct CreatePoolAndMintToken404<'info> {
    #[account(mut)]
    pub dex_configuration_account: Box<Account<'info, CurveConfiguration>>,

    /// CHECK: PDA for SOL curves
    #[account(
        mut,
        seeds = [
            LiquidityPool::SOL_VAULT_PREFIX.as_bytes(), 
            dex_configuration_account.key().as_ref()
        ],
        bump
    )]
    pub pool_sol_curves: AccountInfo<'info>,

    #[account(
        init,
        space = LiquidityPool::ACCOUNT_SIZE,
        payer = payer,
        seeds = [
            LiquidityPool::POOL_SEED_PREFIX.as_bytes(), 
            token_mint.key().as_ref()
        ],
        bump
    )]
    pub pool: Box<Account<'info, LiquidityPool>>,

    #[account(
        init,
        payer = payer,
        mint::decimals = 9,
        mint::authority = payer.key(),
        mint::freeze_authority = payer.key(),
    )]
    pub token_mint: Box<Account<'info, Mint>>,

    #[account(
        init,
        payer = payer,
        associated_token::mint = token_mint,
        associated_token::authority = pool
    )]
    pub pool_token_account: Box<Account<'info, TokenAccount>>,

    /// CHECK: Metadata PDA derived from token mint
    #[account(
        mut,
        seeds = [
            b"metadata", 
            token_metadata_program.key().as_ref(), 
            token_mint.key().as_ref()
        ],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    pub metadata_account: UncheckedAccount<'info>,

    /// CHECK: PDA for SOL vault
    #[account(
        mut,
        seeds = [
            LiquidityPool::SOL_VAULT_PREFIX.as_bytes(), 
            token_mint.key().as_ref()
        ],
        bump
    )]
    pub pool_sol_vault: AccountInfo<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn create_pool_and_mint_token404(
    ctx: Context<CreatePoolAndMintToken404>,
    token_name: String,
    token_symbol: String,
    token_uri: String
) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    let initial_supply = 1_000_000_000 * 10u64.pow(9);

    // Initialize pool
    pool.set_inner(LiquidityPool::new2(
        ctx.accounts.payer.key(),
        ctx.accounts.token_mint.key(),
        ctx.bumps.pool,
    ));

    // Create metadata
    create_metadata_accounts_v3(
        CpiContext::new(
            ctx.accounts.token_metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                metadata: ctx.accounts.metadata_account.to_account_info(),
                mint: ctx.accounts.token_mint.to_account_info(),
                mint_authority: ctx.accounts.payer.to_account_info(),
                update_authority: ctx.accounts.payer.to_account_info(),
                payer: ctx.accounts.payer.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
        ),
        DataV2 {
            name: token_name,
            symbol: token_symbol,
            uri: token_uri,
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        },
        false,
        true,
        None,
    )?;

    // Mint initial supply
    mint_to(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::MintTo {
                mint: ctx.accounts.token_mint.to_account_info(),
                to: ctx.accounts.pool_token_account.to_account_info(),
                authority: ctx.accounts.payer.to_account_info(),
            },
        ),
        initial_supply,
    )?;

    // Set token authorities to pool
    for authority_type in [
        anchor_spl::token::spl_token::instruction::AuthorityType::MintTokens,
        anchor_spl::token::spl_token::instruction::AuthorityType::FreezeAccount,
    ] {
        set_authority(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::SetAuthority {
                    account_or_mint: ctx.accounts.token_mint.to_account_info(),
                    current_authority: ctx.accounts.payer.to_account_info(),
                },
            ),
            authority_type,
            Some(pool.key()),
        )?;
    }


    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.payer.to_account_info(),
                to: ctx.accounts.pool_sol_vault.to_account_info(),
            },
        ),
        INITIAL_LAMPORTS_FOR_POOL,
    )?;
    
    // Update pool state
    pool.total_supply = 1_000_000_000_000_000_000;
    pool.update_reserves(1_000_000_000_000_000_000, 0)?;

    Ok(())
}