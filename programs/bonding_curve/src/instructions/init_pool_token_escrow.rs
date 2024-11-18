use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};
use crate::state::*;

#[derive(Accounts)]
pub struct InitPoolEscrow<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [
            EscrowPool::POOL_SEED_PREFIX.as_bytes(), 
            pool_escrow.token.key().as_ref(), 
            pool_escrow.collection.key().as_ref()
        ],
        bump
    )]
    pub pool_escrow: Box<Account<'info, EscrowPool>>,

    #[account(
        init,
        payer = payer,
        associated_token::mint = token_mint,
        associated_token::authority = pool_escrow,
    )]
    pub pool_escrow_token_account: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub token_mint: Box<Account<'info, Mint>>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn init_pool_escrow(ctx: Context<InitPoolEscrow>) -> Result<()> {
    // Only log in development environment
    msg!("Pool escrow token account initialized: {}", 
        ctx.accounts.pool_escrow_token_account.key()
    );
    
    Ok(())
}