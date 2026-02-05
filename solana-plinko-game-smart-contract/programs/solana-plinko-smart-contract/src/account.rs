use anchor_lang::prelude::*;
use crate::errors::PlinkoError;
use solana_program::keccak;
use solana_program::keccak::hashv;

#[derive(Debug, AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Waiting,
    Processing,
    Finished,
}

impl<> Default for Status {
    fn default() -> Self {
        return Status::Waiting;
    }
}

#[account]
pub struct House {
    pub owner: Pubkey, // House owner
    pub balance: u64, // Total house balance
    pub maximum_payout: u64, // Maximum payout allowed
    pub total_payout: u64, // Total payouts made
    pub withdrawals_pause: bool, // Whether withdrawals are paused
    pub pending_request: u32, // Number of pending random requests
}

impl House {
    pub const LEN: usize =
        8 + //discriminator
        32 + // owner
        8 + // balance
        8 + // maximum_payout
        8 + // total_payout
        1 + // withdrawals_pause
        4; // pending_request
}

#[account]
pub struct PlinkoStatus {
    pub owner: Pubkey, // Owner of the game
    pub platform_fee: u64, // Platform fee for the (300 = 3%)
    pub fee_denominator: u64, // Denominator for fee calculation (10,000 = 100%)
    pub payout_denominator: u64, // Denominator for payout calculation
    pub min_buy_in: u64, // Minimum buy-in amount
    pub max_balls: u8, // Maximum balls per game
    pub odds_locked: bool, // Whether odds are locked
    pub paused: bool, // Whether the game is paused
    pub bucket_weights: Vec<u64>, // Weights of each bucket for random distribution
    pub payouts: Vec<u64>, // Payouts for each bucket
    pub total_games: u64, // Total number of games played
    pub total_volume: u64, // Total volume of bets
    pub total_payouts: u64, // Total payouts made
    pub fee_treasury: Pubkey, // Treasury for platform fees
    pub house_account: Pubkey, // House account for the game
    pub force: [u8; 32],
    pub status: Status,
}

impl PlinkoStatus {
    pub const LEN: usize =
        8 + // discriminator
        32 + // owner
        8 + // platform_fee
        8 + // fee_denominator
        8 + // payout_denominator
        8 + // min_buy_in
        1 + // max_balls
        1 + // odds_locked
        1 + // paused
        4 * 64 + // bucket_weights (max size of 64)
        8 * 64 + // payouts (max size of 64)
        8 + // total_games
        8 + // total_volume
        8 + // total_payouts
        32 + // fee_treasury
        32 + // house_account
        32 +
        1;
}

#[account]
pub struct Game {
    pub game_id: u64, // Unique identifier for the game
    pub player: Pubkey, // Player's public key
    pub bet_amount: u64, // Amount bet by the player
    pub amount_for_house: u64, // Amount allocated for the house
    pub num_balls: u8, // Number of balls played in the game
    pub bet_amount_per_ball: u64, // Amount bet per ball
    pub buckets: Vec<u8>, // Buckets where balls landed
    pub payout: u64, // Amount to be paid out to the player
    pub has_ended: bool, // Whether the game has ended
    pub request_id: u64, // Request ID for VRF randomness
    pub created_at: i64, // Timestamp when the game was created
    pub ended_at: i64, // Timestamp when the game ended
}

impl Game {
    pub const LEN: usize =
        8 + // discriminator
        8 + // game_id
        32 + // player
        8 + // bet_amount
        8 + // amount_for_house
        1 + // num_balls
        8 + // bet_amount_per_ball
        4 +
        60 * 1 + // buckets (max size of 100)
        8 + // payout
        1 + // has_ended
        8 + // request_id
        8 + // created_at
        8; // ended_at
}

#[account]
pub struct UserStats {
    pub user: Pubkey, // User's public key
    pub total_games: u64, // Total number of games played by the user
    pub total_wagered: u64, // Total amount wagered by the user
    pub total_won: u64, // Total amount won by the user
    pub game_ids: Vec<u64>, // List of game IDs played by the user
}

impl UserStats {
    pub const LEN: usize =
        8 + // discriminator
        32 + // user
        8 + // total_games
        8 + // total_wagered
        8 + // total_won
        4 +
        100 * 8; // game_ids (max 100 games per user)
}
