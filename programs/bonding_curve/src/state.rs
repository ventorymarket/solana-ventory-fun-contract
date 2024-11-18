use crate::consts::INITIAL_LAMPORTS_FOR_POOL;
use crate::consts::DECIMAL;
use crate::consts::{current_price_return,calculate_sale_return,calculate_purchase_return};
use crate::errors::CustomError;
use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

#[account]
pub struct CurveConfiguration {
    pub fees: f64,
    pub admin: Pubkey
}

impl CurveConfiguration {
    pub const SEED: &'static str = "CurveConfiguration";

    // Discriminator (8) +Pubkey (32) + f64 (8)
    pub const ACCOUNT_SIZE: usize = 8 + 32 + 8;

    pub fn new(fees: f64,admin: Pubkey) -> Self {
        Self { fees ,admin}
    }
}

#[account]
pub struct LiquidityProvider {
    pub shares: u64, // The number of shares this provider holds in the liquidity pool ( didnt add to contract now )
}

impl LiquidityProvider {
    pub const SEED_PREFIX: &'static str = "LiqudityProvider"; // Prefix for generating PDAs

    // Discriminator (8) + f64 (8)
    pub const ACCOUNT_SIZE: usize = 8 + 8;
}

#[account]
pub struct LiquidityPool {
    pub creator: Pubkey,    // Public key of the pool creator
    pub token: Pubkey,      // Public key of the token in the liquidity pool
    pub total_supply: u64,  // Total supply of liquidity tokens
    pub reserve_token: u64, // Reserve amount of token in the pool
    pub reserve_sol: u64,   // Reserve amount of sol_token in the pool
    pub status: u8,         // 0 buy sell, 1 pool full, 2 add amm
    pub type_c: u8,        // 0 init , 1 erc20, 2 erc404  
    pub bump: u8,           // Nonce for the program-derived address
}

impl LiquidityPool {

    pub const POOL_SEED_PREFIX: &'static str = "liquidity_pool";
    pub const SOL_VAULT_PREFIX: &'static str = "liquidity_sol_vault";

    // Discriminator (8) + Pubkey (32) + Pubkey (32) + totalsupply (8)
    // + reserve one (8) + reserve two (8) + Bump (1)
    pub const ACCOUNT_SIZE: usize = 8 + 32 + 32 + 8 + 8 + 8 + 1 + 1 + 1;

    // Constructor to initialize a LiquidityPool with two tokens and a bump for the PDA
    pub fn new1(creator: Pubkey, token: Pubkey, bump: u8) -> Self {
        Self {
            creator,
            token,
            total_supply: 0_u64,
            reserve_token: 0_u64,
            reserve_sol: 0_u64,
            status: 0_u8,
            type_c: 1_u8,
            bump,
        }
    }
    pub fn new2(creator: Pubkey, token: Pubkey, bump: u8) -> Self {
        Self {
            creator,
            token,
            total_supply: 0_u64,
            reserve_token: 0_u64,
            reserve_sol: 0_u64,
            status: 0_u8,
            type_c: 0_u8,
            bump,
        }
    }
}

pub trait LiquidityPoolAccount<'info> {

    fn calculate_amount_out_token20(&self, sol_amount: u64) -> Result<u64>;

    // Updates the token reserves in the liquidity pool
    fn update_reserves(&mut self, reserve_token: u64, reserve_sol: u64) -> Result<()>;

    // Allows adding liquidity by depositing an amount of two tokens and getting back pool shares
    fn add_liquidity(
        &mut self,
        pool_sol_vault: &mut AccountInfo<'info>,
        authority: &Signer<'info>,
        system_program: &Program<'info, System>,
    ) -> Result<()>;

    // Allows removing liquidity by burning pool shares and receiving back a proportionate amount of tokens
    fn remove_liquidity(
        &mut self,
        token_accounts: (
            &mut Account<'info, Mint>,
            &mut Account<'info, TokenAccount>,
            &mut Account<'info, TokenAccount>,
        ),
        pool_sol_account: &mut AccountInfo<'info>,
        authority: &Signer<'info>,
        bump: u8,
        token_program: &Program<'info, Token>,
        system_program: &Program<'info, System>,
    ) -> Result<()>;

    fn buy(
        &mut self,
        // bonding_configuration_account: &Account<'info, CurveConfiguration>,
        token_accounts: (
            &mut Account<'info, Mint>,
            &mut Account<'info, TokenAccount>,
            &mut Account<'info, TokenAccount>,
        ),
        pool_sol_vault: &mut AccountInfo<'info>,
        pool_sol_curves: &mut AccountInfo<'info>,
        amount: u64,
        authority: &Signer<'info>,
        token_program: &Program<'info, Token>,
        system_program: &Program<'info, System>,
    ) -> Result<()>;

    fn sell(
        &mut self,
        // bonding_configuration_account: &Account<'info, CurveConfiguration>,
        token_accounts: (
            &mut Account<'info, Mint>,
            &mut Account<'info, TokenAccount>,
            &mut Account<'info, TokenAccount>,
        ),
        pool_sol_vault: &mut AccountInfo<'info>,
        pool_sol_curves: &mut AccountInfo<'info>,
        amount: u64,
        bump: u8,
        authority: &Signer<'info>,
        token_program: &Program<'info, Token>,
        system_program: &Program<'info, System>,
    ) -> Result<()>;

    fn transfer_token_from_pool(
        &self,
        from: &Account<'info, TokenAccount>,
        to: &Account<'info, TokenAccount>,
        amount: u64,
        token_program: &Program<'info, Token>,
    ) -> Result<()>;

    fn transfer_token_to_pool(
        &self,
        from: &Account<'info, TokenAccount>,
        to: &Account<'info, TokenAccount>,
        amount: u64,
        authority: &Signer<'info>,
        token_program: &Program<'info, Token>,
    ) -> Result<()>;

    fn transfer_sol_to_pool(
        &self,
        from: &Signer<'info>,
        to: &mut AccountInfo<'info>,
        amount: u64,
        system_program: &Program<'info, System>,
    ) -> Result<()>;

    fn transfer_sol_from_pool(
        &self,
        from: &mut AccountInfo<'info>,
        to: &Signer<'info>,
        amount: u64,
        bump: u8,
        system_program: &Program<'info, System>,
    ) -> Result<()>;
}

impl<'info> LiquidityPoolAccount<'info> for Account<'info, LiquidityPool> {
    
    
    fn calculate_amount_out_token20(&self, sol_amount: u64) -> Result<u64> {
        // Kiểm tra nếu pool rỗng
        if self.reserve_token == 0 || self.reserve_sol == 0 {
            return err!(CustomError::InvalidAmount);
        }

        // Tính toán số lượng token dựa trên Linear Bonding Curve
        let token_amount = (sol_amount as u128)
            .checked_mul(self.reserve_token as u128)
            .ok_or(ProgramError::ArithmeticOverflow)?
            .checked_div(self.reserve_sol as u128)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        // Kiểm tra nếu số lượng token vượt quá reserve
        if token_amount > self.reserve_token as u128 {
            return err!(CustomError::InvalidAmount);
        }

        Ok(token_amount as u64)
    }

    fn update_reserves(&mut self, reserve_token: u64, reserve_sol: u64) -> Result<()> {
        self.reserve_token = reserve_token;
        self.reserve_sol = reserve_sol;
        Ok(())
    }

    fn add_liquidity(
        &mut self,
        pool_sol_vault: &mut AccountInfo<'info>,
        authority: &Signer<'info>,
        system_program: &Program<'info, System>,
    ) -> Result<()> {

        self.transfer_sol_to_pool(
            authority,
            pool_sol_vault,
            INITIAL_LAMPORTS_FOR_POOL,
            system_program,
        )?;

        self.total_supply = 1_000_000_000_000_000_000;
        self.creator = authority.key();
        self.update_reserves( 1_000_000_000_000_000_000, 0)?;
        
        Ok(())
    }

    fn remove_liquidity(
        &mut self,
        token_accounts: (
            &mut Account<'info, Mint>,
            &mut Account<'info, TokenAccount>,
            &mut Account<'info, TokenAccount>,
        ),
        pool_sol_vault: &mut AccountInfo<'info>,
        authority: &Signer<'info>,
        bump: u8,
        token_program: &Program<'info, Token>,
        system_program: &Program<'info, System>,
    ) -> Result<()> {

        let amount_token = token_accounts.1.amount as u64;
        self.transfer_token_from_pool(
            token_accounts.1,
            token_accounts.2,
            amount_token,
            token_program,
        )?;
        // let amount = self.to_account_info().lamports() - self.get_lamports();
        let amount = pool_sol_vault.to_account_info().lamports() as u64;
        self.transfer_sol_from_pool(pool_sol_vault, authority, amount, bump, system_program)?;
        
        emit!(ClaimAddLiquidityRaydium{
            token_address: token_accounts.0.key(),
            amount_sol: amount,
            amount_token: amount_token
        });

        Ok(())
    }

    fn buy(
        &mut self,
        // _bonding_configuration_account: &Account<'info, CurveConfiguration>,
        token_accounts: (
            &mut Account<'info, Mint>,
            &mut Account<'info, TokenAccount>,
            &mut Account<'info, TokenAccount>,
        ),
        pool_sol_vault: &mut AccountInfo<'info>,
        pool_sol_curves: &mut AccountInfo<'info>,
        amount: u64,
        authority: &Signer<'info>,
        token_program: &Program<'info, Token>,
        system_program: &Program<'info, System>,
    ) -> Result<()> {

        if amount == 0 {
            return err!(CustomError::InvalidAmount);
        }
        if self.status != 0{
            return err!(CustomError::PendingAddLiquidity);
        }

        if self.type_c == 0{
            return err!(CustomError::PendingInit);
        }

        msg!("Trying to buy from the pool");

        let bought_amount = self.total_supply  - self.reserve_token ;
        msg!("bought_amount {}", bought_amount);

        let mut amount_eastimate  = amount;

        if bought_amount + amount >= 800_000_000 * DECIMAL{
            amount_eastimate = 800_000_000 * DECIMAL - bought_amount;
        }
        msg!("amount_token_eastimate {}", amount_eastimate);

        let amount_sol = calculate_purchase_return(bought_amount + amount_eastimate,self.reserve_sol,DECIMAL);
        
        msg!("amount_sol_eastimate {}", amount_sol);

        self.reserve_sol += amount_sol;
        self.reserve_token -= amount_eastimate;

        msg!("reserve_sol {}", self.reserve_sol);
        msg!("reserve_token {}", self.reserve_token);

        self.transfer_sol_to_pool(authority, pool_sol_vault, amount_sol, system_program)?;
        self.transfer_sol_to_pool(authority, pool_sol_curves, amount_sol * 10 /1000, system_program)?;
        self.transfer_token_from_pool(
            token_accounts.1,
            token_accounts.2,
            amount_eastimate,
            token_program,
        )?;
        msg!("current_price_return {}",current_price_return(bought_amount + amount_eastimate));

        emit!(BuyEvent20{
            token_address: token_accounts.0.key(),
            user: authority.key(),
            amount_sol: amount_sol,
            amount_token: amount_eastimate,
            currency_price: current_price_return(bought_amount + amount_eastimate)
        });

        if self.reserve_sol >= 20 * DECIMAL || self.reserve_token <= 200_000_000 *  DECIMAL{
            self.status = 1;

            emit!(AcceptFullPoolEvent20{
                token_address: token_accounts.0.key(),
                amount_token_bought: bought_amount + amount_eastimate,
                amount_sol_pool: self.reserve_sol,
                currency_price: current_price_return(bought_amount + amount_eastimate)
            });
        }

        Ok(())
    }

    fn sell(
        &mut self,
        token_accounts: (
            &mut Account<'info, Mint>,
            &mut Account<'info, TokenAccount>,
            &mut Account<'info, TokenAccount>,
        ),
        pool_sol_vault: &mut AccountInfo<'info>,
        pool_sol_curves: &mut AccountInfo<'info>,
        amount: u64,
        bump: u8,
        authority: &Signer<'info>,
        token_program: &Program<'info, Token>,
        system_program: &Program<'info, System>,
    ) -> Result<()> {

        if amount == 0 {
            return err!(CustomError::InvalidAmount);
        }

        if self.status != 0{
            return err!(CustomError::PendingAddLiquidity);
        }

        if self.type_c == 0{
            return err!(CustomError::PendingInit);
        }

        let bought_amount = self.total_supply - self.reserve_token ;
        msg!("bought_amount: {}", bought_amount);
        
        let mut amount_eastimate = amount;

        if bought_amount <= amount{
            amount_eastimate = bought_amount;
        }
        msg!("amount_eastimate: {}", amount_eastimate);

        let bought_amount_supply_mint = bought_amount - amount_eastimate;
        msg!("bought_amount_supply_mint: {}", bought_amount_supply_mint);
        
        let amount_sol_out = calculate_sale_return(bought_amount_supply_mint,self.reserve_sol,DECIMAL);
        msg!("amount_sol_out {}",amount_sol_out);
    
        self.reserve_token += amount_eastimate;
        self.reserve_sol -= amount_sol_out;

        msg!("reserve_sol {}", self.reserve_sol);
        msg!("reserve_token {}", self.reserve_token);

        self.transfer_token_to_pool(
            token_accounts.2,
            token_accounts.1,
            amount_eastimate as u64,
            authority,
            token_program,
        )?;

        self.transfer_sol_from_pool(pool_sol_vault, authority, amount_sol_out, bump, system_program)?;
        self.transfer_sol_to_pool(authority, pool_sol_curves, amount_sol_out * 10 /1000, system_program)?;

        msg!("current_price_return {}",current_price_return(bought_amount_supply_mint));

        emit!(SellEvent20{
            token_address: token_accounts.0.key(),
            user: authority.key(),
            amount_token: amount_eastimate,
            amount_sol: amount_sol_out,
            currency_price: current_price_return(bought_amount_supply_mint)
        });

        Ok(())
    }

    fn transfer_token_from_pool(
        &self,
        from: &Account<'info, TokenAccount>,
        to: &Account<'info, TokenAccount>,
        amount: u64,
        token_program: &Program<'info, Token>,
    ) -> Result<()> {
        token::transfer(
            CpiContext::new_with_signer(
                token_program.to_account_info(),
                token::Transfer {
                    from: from.to_account_info(),
                    to: to.to_account_info(),
                    authority: self.to_account_info(),
                },
                &[&[
                    LiquidityPool::POOL_SEED_PREFIX.as_bytes(),
                    self.token.key().as_ref(),
                    &[self.bump],
                ]],
            ),
            amount,
        )?;
        Ok(())
    }

    fn transfer_token_to_pool(
        &self,
        from: &Account<'info, TokenAccount>,
        to: &Account<'info, TokenAccount>,
        amount: u64,
        authority: &Signer<'info>,
        token_program: &Program<'info, Token>,
    ) -> Result<()> {
        token::transfer(
            CpiContext::new(
                token_program.to_account_info(),
                token::Transfer {
                    from: from.to_account_info(),
                    to: to.to_account_info(),
                    authority: authority.to_account_info(),
                },
            ),
            amount,
        )?;
        Ok(())
    }

    fn transfer_sol_from_pool(
        &self,
        from: &mut AccountInfo<'info>,
        to: &Signer<'info>,
        amount: u64,
        bump: u8,
        system_program: &Program<'info, System>,
    ) -> Result<()> {
        // let pool_account_info = self.to_account_info();

        system_program::transfer(
            CpiContext::new_with_signer(
                system_program.to_account_info(),
                system_program::Transfer {
                    from: from.clone(),
                    to: to.to_account_info().clone(),
                },
                &[&[
                    LiquidityPool::SOL_VAULT_PREFIX.as_bytes(),
                    self.token.key().as_ref(),
                    // LiquidityPool::POOL_SEED_PREFIX.as_bytes(),
                    // self.token.key().as_ref(),
                    &[bump],
                ]],
            ),
            amount,
        )?;
        Ok(())
    }

    fn transfer_sol_to_pool(
        &self,
        from: &Signer<'info>,
        to: &mut AccountInfo<'info>,
        amount: u64,
        system_program: &Program<'info, System>,
    ) -> Result<()> {
        // let pool_account_info = self.to_account_info();

        system_program::transfer(
            CpiContext::new(
                system_program.to_account_info(),
                system_program::Transfer {
                    from: from.to_account_info(),
                    to: to.to_account_info(),
                },
            ),
            amount,
        )?;
        Ok(())
    }
}

#[event]
pub struct BuyEvent20 {
    pub token_address: Pubkey,
    pub user: Pubkey,
    pub amount_sol: u64,
    pub amount_token: u64,
    pub currency_price: u64
}

#[event]
pub struct TransferNft {
    pub collection: Pubkey,
    pub token: Pubkey,
    pub pool_escrow: Pubkey,
    pub from: Pubkey,
    pub to: Pubkey,
    pub nft: Pubkey
}

#[event]
pub struct ClaimAddLiquidityRaydium {
    pub token_address: Pubkey,
    pub amount_sol: u64,
    pub amount_token: u64
}

pub fn emit_transfer_event(
    collection: Pubkey,
    token: Pubkey,
     pool_escrow: Pubkey,
     from: Pubkey,
     to: Pubkey,
     nft: Pubkey
) {
    emit!(TransferNft {
        collection,
        token,
        pool_escrow,
        from,
        to,
        nft
    });
}

#[event]
pub struct SellEvent20 {
    pub token_address: Pubkey,
    pub user: Pubkey,
    pub amount_token: u64,
    pub amount_sol: u64,
    pub currency_price: u64
}

#[event]
pub struct AcceptFullPoolEvent20 {
    pub token_address: Pubkey,
    pub amount_token_bought: u64,
    pub amount_sol_pool: u64,
    pub currency_price: u64
}


#[account]
pub struct EscrowPool {
    //32 the collection account
    pub collection: Pubkey,
    //32 the token to be dispensed
    pub token: Pubkey,
    //4 the NFT name
    pub name: String,
    //4 the NFT symbol
    pub symbol: String,
    //4 the base uri for the NFT metadata
    pub base_uri: String,
    //2 the max index of NFTs that append to the uri
    pub uri_size: u16,
    //4 + uri_size the minimum index of NFTs that append to the uri
    pub path: Vec<u16>,
    pub path_max: u16,
    pub path_min: u16,
    //8 the token cost to swap
    pub amount: u64,
    //8 the total number of swaps
    pub count: u16,
    //1 escrow bump
    pub bump: u8,
}

impl EscrowPool {

    pub const POOL_SEED_PREFIX: &'static str = "escrow_pool";
    pub const SOL_VAULT_PREFIX: &'static str = "escrow_sol_vault";

    // Discriminator (8) + Pubkey (32) + Pubkey (32)
    pub const DISCRIMINATOR: usize = 8;
    pub const PUBKEY_SIZE: usize = 32;
    pub const U64_SIZE: usize = 8;
    pub const U16_SIZE: usize = 2;
    pub const U8_SIZE: usize = 1;
    pub const VEC_PREFIX_SIZE: usize = 4;

    pub fn get_space() -> usize {
        Self::DISCRIMINATOR +    // Anchor discriminator
        Self::PUBKEY_SIZE +     // collection
        Self::PUBKEY_SIZE +     // token
        4 +      // name string
        4 +    // symbol string
        4 +  // base_uri string
        Self::U16_SIZE +        // uri_size
        2008 +     // path vector
        Self::U16_SIZE + 
        Self::U16_SIZE +  
        Self::U64_SIZE +        // amount
        Self::U16_SIZE +        // count
        Self::U8_SIZE           // bump
    }


    // Constructor to initialize a LiquidityPool with two tokens and a bump for the PDA
    pub fn new(
        collection: Pubkey,
        token: Pubkey,
        name: String,
        symbol: String,
        base_uri: String,
        uri_size: u16,
        path: Vec<u16>,
        bump: u8
    ) -> Self 
    {
        Self {
            collection,
            token,
            name,
            symbol,
            base_uri,
            uri_size,
            path,
            path_min: 0,
            path_max: 0,
            amount: 0,
            count: 0,  // Initialize count as 0
            bump,
        }
    }
}

pub trait EscrowPoolAccount<'info> {
    
}

impl<'info> EscrowPoolAccount<'info> for Account<'info, EscrowPool> {
    
}
