use anchor_lang::prelude::*;

#[error_code]
pub enum CrashError {
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Invalid multiplier")]
    InvalidMultiplier,
    #[msg("Invalid round state")]
    InvalidRoundState,
    #[msg("Betting closed")]
    BettingClosed,
    #[msg("Betting still open")]
    BettingStillOpen,
    #[msg("Round not revealed")]
    RoundNotRevealed,
    #[msg("Already claimed")]
    AlreadyClaimed,
    #[msg("No cashout target")]
    NoCashoutTarget,
    #[msg("Bet lost")]
    BetLost,
    #[msg("Math overflow")]
    MathOverflow,
    #[msg("Commit-reveal mismatch")]
    BadReveal,
    #[msg("Paused")]
    Paused,
    #[msg("Insufficient vault funds")]
    InsufficientVault,
    #[msg("Edge too high")]
    EdgeTooHigh,
    #[msg("Invalid time bounds")]
    InvalidTime,
    #[msg("Already revealed")]
    AlreadyRevealed,
}
