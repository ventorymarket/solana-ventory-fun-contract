use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use crate::{
    consts::{DEV_WALLET_OWNER,POOL_RAYDIUM_WALLET_OWNER},
    errors::CustomError, 
    state::{CurveConfiguration,LiquidityPool, LiquidityPoolAccount}
};

#[derive(Accounts)]
pub struct RemoveLiquidity<'info> {
    #[account(
        mut,
        seeds = [CurveConfiguration::SEED.as_bytes()],
        bump
    )]
    pub dex_configuration_account: Box<Account<'info, CurveConfiguration>>,

    #[account(
        mut,
        seeds = [
            LiquidityPool::POOL_SEED_PREFIX.as_bytes(),
            token_mint.key().as_ref()
        ],
        bump = pool.bump
    )]
    pub pool: Box<Account<'info, LiquidityPool>>,

    #[account(mut)]
    pub token_mint: Box<Account<'info, Mint>>,

    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = pool
    )]
    pub pool_token_account: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = token_mint,
        associated_token::authority = user,
    )]
    pub user_token_account: Box<Account<'info, TokenAccount>>,

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
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn remove_liquidity(ctx: Context<RemoveLiquidity>, bump: u8) -> Result<()> {
    let user_key = ctx.accounts.user.key();
    let pool = &mut ctx.accounts.pool;

    // Validate authority
    require!(
        ctx.accounts.dex_configuration_account.admin == user_key || 
        user_key == DEV_WALLET_OWNER || 
        user_key == POOL_RAYDIUM_WALLET_OWNER
        ,CustomError::NotOnwer
    );

    // // Validate pool status
    // if pool.status == 2 {
    //     return Err(CustomError::ClaimedBondingCurvers.into());
    // };

    require!(pool.status == 1, CustomError::PendingBondingCurvers);

    // Remove liquidity
    pool.remove_liquidity(
        (
            &mut *ctx.accounts.token_mint,
            &mut *ctx.accounts.pool_token_account,
            &mut *ctx.accounts.user_token_account,
        ),
        &mut ctx.accounts.pool_sol_vault,
        &ctx.accounts.user,
        bump,
        &ctx.accounts.token_program,
        &ctx.accounts.system_program,
    )?;

    pool.status = 2;

    
    Ok(())
}