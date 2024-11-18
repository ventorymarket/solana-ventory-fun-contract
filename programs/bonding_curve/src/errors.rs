use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {

    #[msg("Invalid amount to swap")]
    InvalidAmount,

    #[msg("Invalid fee")]
    InvalidFee,

    #[msg("NotOnwer")]
    NotOnwer,
    
    #[msg("Pending add liquidity,")]
    PendingAddLiquidity,

    #[msg("Pending Bonding Curvers,")]
    PendingBondingCurvers,

    #[msg("Claimed Bonding Curvers,")]
    ClaimedBondingCurvers,

    #[msg("Pending Init")]
    PendingInit,

    #[msg("Limit Mint 1000 nft")]
    LimitMintNft,

    #[msg("Invalid Collection")]
    InvalidCollection,

    #[msg("Collection Not Verified")]
    CollectionNotVerified,

    #[msg("Invalid Sum = 1000")]
    InvalidSum,

    #[msg("Empty Array Uri")]
    EmptyArrayUri
}
