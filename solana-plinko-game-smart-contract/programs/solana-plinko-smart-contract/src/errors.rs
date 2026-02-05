use anchor_lang::prelude::*;

#[error_code]
pub enum PlinkoError {
    #[msg("Invalid number of balls")]
    InvalidNumberOfBalls,

    #[msg("Game ID already used")]
    GameIdAlreadyUsed,

    #[msg("Invalid bet amount")]
    InvalidBetAmount,

    #[msg("Invalid value")]
    InvalidValue,

    #[msg("Game is paused")]
    GamePaused,

    #[msg("Only owner can call this function")]
    OnlyOwner,

    #[msg("Only VRF can call this function")]
    OnlyVrf,

    #[msg("Odds are locked")]
    OddsLocked,

    #[msg("Invalid bucket weights and payouts length")]
    InvalidLength,

    #[msg("Game not found")]
    GameNotFound,

    #[msg("Game already ended")]
    GameAlreadyEnded,

    #[msg("Invalid request ID")]
    InvalidRequestId,

    #[msg("Cannot exceed 5% platform fee")]
    PlatformFeeTooHigh,

    #[msg("Cannot exceed 100% VRF fee")]
    VrfFeeTooHigh,

    #[msg("Cannot exceed 100 balls")]
    MaxBallsTooHigh,

    #[msg("Invalid bucket index")]
    InvalidBucketIndex,

    #[msg("Insufficient funds")]
    InsufficientFunds,

    #[msg("Invalid random words")]
    InvalidRandomWords,

    #[msg("Request ID not found")]
    RequestIdNotFound,

    #[msg("Randomness is still being fulfilled")]
    StillProcessing,
}
