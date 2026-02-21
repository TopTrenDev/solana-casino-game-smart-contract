use anchor_lang::prelude::*;

#[error_code]
pub enum CoinflipError {
    #[msg("Side must be 0 (heads) or 1 (tails)")]
    InvalidSide,
    #[msg("Amount must be positive")]
    InvalidAmount,
    #[msg("Fee must be between 0 and 10000 bps")]
    InvalidFee,
    #[msg("Game already resolved")]
    AlreadyResolved,
}
