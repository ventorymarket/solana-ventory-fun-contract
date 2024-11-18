use anchor_lang::solana_program::{pubkey, pubkey::Pubkey};

pub const INITIAL_LAMPORTS_FOR_POOL: u64 = 1_000_000;   // 0.005SOL
pub const DECIMAL: u64 = 1_000_000_000; 

// Math Constants for Calculations
// const BASE_SUPPLY: u128 = 800_000_000;
// const BASE_SUPPLY_PLUS_ONE: u128 = 800_000_001;
// const DENOMINATOR: u128 = BASE_SUPPLY * BASE_SUPPLY_PLUS_ONE;
// const MULTIPLIER: u128 = 20;
// const CURRENT_PRICE_MULTIPLIER: u128 = 40;

pub const DEV_WALLET_OWNER: Pubkey = pubkey!("8tVWVM44VoScdkrsQynYPAges3VxZfWYrTEy3VqpH5uv");
pub const POOL_RAYDIUM_WALLET_OWNER: Pubkey = pubkey!("Cp3FfQG4LgWgeNZoui7CQX6Cdy6H9mVYxPFGXu2Mc8MJ");

pub fn calculate_purchase_return(
    total_supply_token: u64,
    pool_balance_sol: u64,
    decimal: u64
) -> u64 {
    let total_supply = total_supply_token as u128;
    let pool_balance = pool_balance_sol as u128;
    let decimal = decimal as u128;
    
    let denominator = 800000000u128 * 800000001u128;
    
    let step1 = total_supply * (total_supply + decimal); 
    let step2 = (step1 * 20) / denominator; 
    let step3 = step2 / decimal;
    
    let result = if step3 > pool_balance {
        step3 - pool_balance
    } else {
        0
    };
    
    // Convert back to u64, với check overflow
    result.try_into().unwrap_or(u64::MAX)

}


pub fn calculate_sale_return(
    total_supply_token: u64,
    pool_balance_sol: u64,
    decimal: u64
) -> u64 {
    let total_supply = total_supply_token as u128;
    let pool_balance = pool_balance_sol as u128;
    let decimal = decimal as u128;
    
    let denominator = 800_000_000u128 * 800_000_001u128;
    
    let step1 = total_supply * (total_supply + decimal);
    let step2 = (step1 * 20) / denominator;
    let step3 = step2 / decimal;
    
    let result = if pool_balance > step3 {
        pool_balance - step3
    } else {
        0
    };
    
    // Convert về u64 an toàn
    result.try_into().unwrap_or(u64::MAX)

}

pub fn current_price_return(total_supply: u64) -> u64 {
     // y =  x * a  = x * 2 * 20sol / 800tr * (800tr + 1 )
    let supply = total_supply as u128;

    let denominator = 800_000_000u128 * 800_000_001u128;
    
    let numerator = supply * 40 ;
    
    let result = numerator / denominator;
    
    result.try_into().unwrap_or(u64::MAX)
}