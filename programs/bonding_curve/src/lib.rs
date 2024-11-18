use anchor_lang::prelude::*;

pub mod errors;
pub mod utils;
pub mod instructions;
pub mod state;
pub mod consts;

use crate::instructions::*;

declare_id!("2sMZa6c9j5bRBeeLX3sc69XdjWuxDDocZQBds7r9xGy8");

#[program]
pub mod bonding_curve {
    use super::*;

    pub fn initialize(ctx: Context<InitializeCurveConfiguration>, fee: f64,admin: Pubkey) -> Result<()> {
        instructions::initialize(ctx, fee,admin)
    }

    pub fn update_initialize(ctx: Context<UpdateCurveConfiguration>, fee: f64,admin: Pubkey) -> Result<()> {
        instructions::update_initialize(ctx, fee,admin)
    }

    pub fn claim(ctx: Context<UpdateCurveConfiguration>,bump: u8) -> Result<()> {
        instructions::claim(ctx,bump)
    }

    pub fn buy(ctx: Context<Buy>, amount: u64) -> Result<()> {
        instructions::buy(ctx, amount)
    }

    pub fn sell(ctx: Context<Sell>, amount: u64, bump: u8) -> Result<()> {
        instructions::sell(ctx, amount, bump)
    }

    pub fn create_pool_token20(
        ctx: Context<CreatePoolAndMintToken>,
        token_name: String,
        token_symbol: String,
        token_uri: String,
    ) -> Result<()> {
        instructions::create_pool_and_mint_token20(ctx, token_name, token_symbol,token_uri)
    }

    pub fn create_pool_token404(
        ctx: Context<CreatePoolAndMintToken404>,
        token_name: String,
        token_symbol: String,
        token_uri: String
    ) -> Result<()> {
        instructions::create_pool_and_mint_token404(ctx, token_name, token_symbol,token_uri)
    }

    pub fn init_pool_token404(
        ctx: Context<InitToken404>,
        token_name: String,
        token_symbol: String,
        token_uri: String,
        base_uri: String,
        uri_size: u16,
        path: Vec<u16>,
    ) -> Result<()> {
        instructions::init_token404(ctx, token_name, token_symbol,token_uri,base_uri,uri_size,path)
    }

    pub fn init_pool_escrow(ctx: Context<InitPoolEscrow>) -> Result<()> {
        instructions::init_pool_escrow(ctx)
    }

    pub fn mint_nft(ctx: Context<MintAndVerifyNFT>) -> Result<()> {
        instructions::mint_and_verify(ctx)
    }

    pub fn swap_token_to_nft(ctx: Context<SwapTokenToNFT>) -> Result<()> {
        instructions::transfer_token_and_claim_nft(ctx)
    }

    pub fn swap_nft_to_token(ctx: Context<SwapNFTToToken>) -> Result<()> {
        instructions::transfer_nft_and_claim_token(ctx)
    }

    pub fn add_liquidity_raydium(ctx: Context<RemoveLiquidity>, bump: u8) -> Result<()> {
        instructions::remove_liquidity(ctx,bump)
    }

}
