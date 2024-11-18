use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use crate::state::{CurveConfiguration, LiquidityPool, LiquidityPoolAccount};

#[derive(Accounts)]
pub struct Buy<'info> {
    #[account(
        mut,
        seeds = [CurveConfiguration::SEED.as_bytes()],
        bump
    )]
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

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = token_mint,
        associated_token::authority = user,
    )]
    pub user_token_account: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub user: Signer<'info>,
    // pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}


pub fn buy(ctx: Context<Buy>, amount: u64) -> Result<()> {
    let pool = &mut ctx.accounts.pool;

    pool.buy(
        (
            &mut *ctx.accounts.token_mint,
            &mut *ctx.accounts.pool_token_account,
            &mut *ctx.accounts.user_token_account,
        ),
        &mut ctx.accounts.pool_sol_vault,
        &mut ctx.accounts.pool_sol_curves,
        amount,
        &ctx.accounts.user,
        &ctx.accounts.token_program,
        &ctx.accounts.system_program,
    )?;
    Ok(())
}