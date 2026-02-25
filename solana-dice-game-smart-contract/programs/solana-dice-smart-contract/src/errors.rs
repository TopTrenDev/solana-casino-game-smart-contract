use anchor_lang::prelude::*;

#[error_code]
pub enum GameError {
    #[msg("Invalid bet amount")]
    InvalidBetAmount,

    #[msg("Invalid bet amount violating MaxWinAmount")]
    InvalidBetAmountMaxWinAmountViolation,

    #[msg("Insufficient User SOL Balance")]
    InsufficientUserBalance,

    #[msg("Insufficient Casino Bank SOL Balance")]
    InsufficientCasinoVault,

    #[msg("Mismatching Round Number")]
    RoundNumMismatch,
    
    #[msg("Not allowed to double bet")]
    NotAllowedDoubleBet,

    #[msg("Not Original Player")]
    NotOriginalPlayer,

    #[msg("Not Allowed Game Status")]
    NotAllowedStatus,

    #[msg("Invalid RTP")]
    InvalidRtp,

    #[msg("Only Operation Admin can call this")]
    UnauthorizedOperator,

    #[msg("Only Financial Admin can call this")]
    UnauthorizedFinanceAdmin,

    #[msg("Only Update Admin can call this")]
    UnauthorizedUpdateAdmin,
    
    #[msg("Invalid Target Number")]
    InvalidTargetNumber,
}
