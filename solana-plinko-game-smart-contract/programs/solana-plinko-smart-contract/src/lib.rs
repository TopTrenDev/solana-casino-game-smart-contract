use anchor_lang::prelude::*;

declare_id!("7dzjQ2uoBb9dDC6S4bdAk7rynABaBWrXWaXkp4xBicuv");

pub mod account;
pub mod errors;
pub mod instructions;
pub mod misc;
pub mod utils;

use crate::instructions::*;

#[program]
pub mod solana_plinko_smart_contract {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        platform_fee: u64,
        min_buy_in: u64,
        max_balls: u8
    ) -> Result<()> {
        initialize::handler(ctx, platform_fee, min_buy_in, max_balls)
    }

    pub fn set_payout(
        ctx: Context<SetPayout>,
        bucket_weights: Vec<u64>,
        payouts: Vec<u64>
    ) -> Result<()> {
        set_payout::handler(ctx, bucket_weights, payouts)
    }

    pub fn lock_odds(ctx: Context<LockOdds>) -> Result<()> {
        lock_odds::handler(ctx)
    }

    pub fn play_game(
        ctx: Context<PlayGame>,
        force: [u8; 32],
        game_id: u64,
        num_balls: u8,
        user_bet_amount: u64
    ) -> Result<()> {
        play_game::handler(ctx, force, game_id, num_balls, user_bet_amount)
    }

    pub fn fulfill_random_words(
        ctx: Context<FulFillRandomWords>,
        force: [u8; 32],
        game_id: u64,
        request_id: u64
    ) -> Result<()> {
        fulfill_random_words::handler(ctx, force, game_id, request_id)
    }
}
