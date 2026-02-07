// NOTE: This is a sanitized version for public release
// Critical security-sensitive implementations have been removed
// Replace placeholders with actual implementations before deployment
use anchor_lang::prelude::*;
use crate::instructions::initialize;
use crate::instructions::create_game;
use crate::instructions::join_game;
use crate::instructions::set_winner;
use crate::instructions::winner_payout;

pub mod instructions;
pub mod constants;
pub mod errors;
pub mod state;
pub mod utils;

use instructions::{
    initialize::*,
    create_game::*,
    join_game::*,
    set_winner::*,
    winner_payout::*,
};

declare_id!("3x4QwXtG3mCjYe6WYHuHacmHqkJjcNo3znkmprCESxzB");

#[program]
pub mod solana_jackpot_smart_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, input: ConfigInput) -> Result<()> {
        initialize::handler(ctx, input)
    }

    pub fn create_game(ctx: Context<CreateGame>, force: [u8; 32], roundIndex: u64) -> Result<()> {
        create_game::handler(ctx, force, roundIndex)
    }

    pub fn join_game(ctx: Context<JoinGame>, roundIndex: u64, amount: u64) -> Result<()> {
        join_game::handler(ctx, roundIndex, amount)
    }

    pub fn set_winner(ctx: Context<SetWinner>, force: [u8; 32], roundIndex: u64) -> Result<()> {
        set_winner::handler(ctx, force, roundIndex)
    }

    pub fn winner_payout(ctx: Context<WinnerPayout>, roundIndex: u64) -> Result<()> {
        winner_payout::handler(ctx, roundIndex)
    }
}
