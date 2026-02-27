use anchor_lang::prelude::*;

mod errors;
mod instructions;
mod state;
mod utils;

pub use errors::*;
pub use state::*;

pub const HOUSE_SEED: &[u8] = b"house";
pub const VAULT_SEED: &[u8] = b"vault";
pub const ROUND_SEED: &[u8] = b"round";
pub const BET_SEED: &[u8] = b"bet";

declare_id!("CrASh1111111111111111111111111111111111111");

#[program]
pub mod crash_house {
    use super::*;

    pub fn initialize_house(ctx: Context<instructions::InitializeHouse>, edge_bps: u16) -> Result<()> {
        instructions::house::initialize_house(ctx, edge_bps)
    }

    pub fn set_paused(ctx: Context<instructions::SetPaused>, paused: bool) -> Result<()> {
        instructions::house::set_paused(ctx, paused)
    }

    pub fn house_deposit(ctx: Context<instructions::HouseDeposit>, amount: u64) -> Result<()> {
        instructions::house::house_deposit(ctx, amount)
    }

    pub fn house_withdraw(ctx: Context<instructions::HouseWithdraw>, amount: u64) -> Result<()> {
        instructions::house::house_withdraw(ctx, amount)
    }

    pub fn start_round(
        ctx: Context<instructions::StartRound>,
        round_id: u64,
        commit_hash: [u8; 32],
        betting_ends_at: i64,
        round_ends_at: i64,
    ) -> Result<()> {
        instructions::round::start_round(ctx, round_id, commit_hash, betting_ends_at, round_ends_at)
    }

    pub fn close_betting(ctx: Context<instructions::CloseBetting>) -> Result<()> {
        instructions::round::close_betting(ctx)
    }

    pub fn place_bet(
        ctx: Context<instructions::PlaceBet>,
        amount_lamports: u64,
        auto_cashout_x100: Option<u32>,
    ) -> Result<()> {
        instructions::bet::place_bet(ctx, amount_lamports, auto_cashout_x100)
    }

    pub fn manual_cashout(ctx: Context<instructions::ManualCashout>, cashout_x100: u32) -> Result<()> {
        instructions::bet::manual_cashout(ctx, cashout_x100)
    }

    pub fn reveal_and_settle_round(
        ctx: Context<instructions::RevealRound>,
        server_seed: [u8; 32],
        nonce: u64,
    ) -> Result<()> {
        instructions::round::reveal_and_settle_round(ctx, server_seed, nonce)
    }

    pub fn claim(ctx: Context<instructions::Claim>) -> Result<()> {
        instructions::bet::claim(ctx)
    }
}
