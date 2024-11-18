use crate::{errors::CustomError, state::*};
use anchor_lang::{prelude::*, system_program};

use crate::consts::DEV_WALLET_OWNER;


#[derive(Accounts)]
pub struct InitializeCurveConfiguration<'info> {
    #[account(
        init,
        space = CurveConfiguration::ACCOUNT_SIZE,
        payer = admin,
        seeds = [CurveConfiguration::SEED.as_bytes()],
        bump,
    )]
    pub dex_configuration_account: Box<Account<'info, CurveConfiguration>>,
    /// CHECK: Safe due to PDA validation
    #[account(
        mut,
        seeds = [LiquidityPool::SOL_VAULT_PREFIX.as_bytes(), dex_configuration_account.key().as_ref()],
        bump
    )]
    pub pool_sol_curves: AccountInfo<'info>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateCurveConfiguration<'info> {
    #[account(mut)]
    pub dex_configuration_account: Box<Account<'info, CurveConfiguration>>,
    /// CHECK: Safe due to PDA validation
    #[account(
        mut,
        seeds = [LiquidityPool::SOL_VAULT_PREFIX.as_bytes(), dex_configuration_account.key().as_ref()],
        bump
    )]
    pub pool_sol_curves: AccountInfo<'info>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn initialize(
    ctx: Context<InitializeCurveConfiguration>,
    fees: f64,
    admin: Pubkey
) -> Result<()> {
    require!(fees >= 0_f64 && fees <= 100_f64, CustomError::InvalidFee);
    
    ctx.accounts.dex_configuration_account.set_inner(
        CurveConfiguration::new(fees, admin)
    );
    Ok(())
}

pub fn update_initialize(
    ctx: Context<UpdateCurveConfiguration>,
    fees: f64,
    admin: Pubkey
) -> Result<()> {

    let user_key = ctx.accounts.user.key();
    require!(
        ctx.accounts.dex_configuration_account.admin == user_key || 
        user_key == DEV_WALLET_OWNER,
        CustomError::NotOnwer
    );
    
    ctx.accounts.dex_configuration_account.set_inner(
        CurveConfiguration::new(fees, admin)
    );
    Ok(())
}

pub fn claim(
    ctx: Context<UpdateCurveConfiguration>,
    bump: u8
) -> Result<()> {
    let user_key = ctx.accounts.user.key();
   
    // Validate authority
    require!(
        ctx.accounts.dex_configuration_account.admin == user_key || 
        user_key == DEV_WALLET_OWNER,
        CustomError::NotOnwer
    );

    let balance = ctx.accounts.pool_sol_curves.lamports();
    
    system_program::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.pool_sol_curves.clone(),
                to: ctx.accounts.user.to_account_info().clone(),
            },
            &[&[
                LiquidityPool::SOL_VAULT_PREFIX.as_bytes(),
                ctx.accounts.dex_configuration_account.key().as_ref(),
                &[bump],
            ]],
        ),
        balance,
    )
}