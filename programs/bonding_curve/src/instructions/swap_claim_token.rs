use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::MetadataAccount,
    token::{ Mint, Token, TokenAccount},
};

use anchor_spl::metadata::mpl_token_metadata::ID as MetadataProgramID;
pub use anchor_lang::solana_program::sysvar::instructions::ID as INSTRUCTIONS_ID;
use crate::state::*;
use crate::errors::CustomError;


#[derive(Accounts)]
pub struct SwapNFTToToken<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(
        mut,
        seeds = [EscrowPool::POOL_SEED_PREFIX.as_bytes(), token_mint.key().as_ref(), pool_escrow.collection.key().as_ref()],
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

    #[account(mut)]
    pub nft_mint: Box<Account<'info, Mint>>,
    
    #[account(
        mut,
        seeds = [
            b"metadata",
            MetadataProgramID.as_ref(),
            nft_mint.key().as_ref()
        ],
        seeds::program = MetadataProgramID,
        bump
    )]
    pub nft_metadata: Box<Account<'info, MetadataAccount>>,

    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = user,
    )]
    pub user_nft_account: Box<Account<'info, TokenAccount>>,
    
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = nft_mint,
        associated_token::authority = pool_escrow
    )]
    pub pool_escrow_nft_account: Box<Account<'info, TokenAccount>>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    
}

pub fn transfer_nft_and_claim_token(ctx: Context<SwapNFTToToken>) -> Result<()> {

    let pool_escrow = &mut ctx.accounts.pool_escrow;
    
    // Validate NFT collection
    if let Some(collection) = &ctx.accounts.nft_metadata.collection {
        require!(
            collection.key == pool_escrow.collection.key(),
            CustomError::InvalidCollection
        );
        require!(
            collection.verified,
            CustomError::CollectionNotVerified
        );
    }

      
    anchor_spl::token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::Transfer {
                from: ctx.accounts.pool_escrow_token_account.to_account_info(),
                to: ctx.accounts.user_token_account.to_account_info(),
                authority: ctx.accounts.pool_escrow.to_account_info(),
            },
            &[&[
                EscrowPool::POOL_SEED_PREFIX.as_bytes(),
                ctx.accounts.token_mint.key().as_ref(),
                ctx.accounts.pool_escrow.collection.key().as_ref(),
                &[ctx.bumps.pool_escrow],
            ]],
        ),
        1_000_000 * 1_000_000_000,
    )?;
      
    anchor_spl::token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::Transfer {
                from: ctx.accounts.user_nft_account.to_account_info(),
                to: ctx.accounts.pool_escrow_nft_account.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            }
        ),
        1 // amount = 1 vì NFT luôn có số lượng là 1
    )?;

        
    ctx.accounts.pool_escrow.amount -= 1_000_000 * 1_000_000_000;
    
    emit_transfer_event(
        ctx.accounts.pool_escrow.collection.key(),
        ctx.accounts.pool_escrow.token.key(),
        ctx.accounts.pool_escrow.key(),
        ctx.accounts.user.key(),
        ctx.accounts.pool_escrow.key(),
        ctx.accounts.nft_mint.key()
    );

    Ok(())
}