use anchor_lang::prelude::*;
use solana_program::pubkey::Pubkey;

pub mod account;
pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;
pub mod utils;

use account::*;
use instructions::*;

declare_id!("BqQfYq22b1JFo2aDicPjhPAJisuLgwbLu38MiiS5XM8X");

#[program]
pub mod dice {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        operate_admin: Pubkey,
        financial_admin: Pubkey,
        update_admin: Pubkey,
    ) -> Result<()> {
        instructions::initialize::handler(ctx, operate_admin, financial_admin, update_admin)
    }

    pub fn play_game(
        ctx: Context<PlayGame>,
        target_number: u8,
        is_under: bool,
        bet_amount: u64,
        game_session_id: u64,
    ) -> Result<()> {
        instructions::play_game::handler(ctx, target_number, is_under, bet_amount, game_session_id)
    }

    pub fn set_result(ctx: Context<SetResult>, is_win: bool, game_session_id: u64) -> Result<()> {
        instructions::set_result::handler(ctx, is_win, game_session_id)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        instructions::withdraw::handler(ctx, amount)
    }

    pub fn set_rtp(ctx: Context<SetGlobalPool>, new_rtp: u64) -> Result<()> {
        instructions::admin::set_rtp(ctx, new_rtp)
    }

    pub fn set_max_win_amount(ctx: Context<SetGlobalPool>, new_max_win_amount: u64) -> Result<()> {
        instructions::admin::set_max_win_amount(ctx, new_max_win_amount)
    }

    pub fn set_min_bet_amount(ctx: Context<SetGlobalPool>, new_min_bet_amount: u64) -> Result<()> {
        instructions::admin::set_min_bet_amount(ctx, new_min_bet_amount)
    }

    pub fn set_min_num(ctx: Context<SetGlobalPool>, new_min_num: u8) -> Result<()> {
        instructions::admin::set_min_num(ctx, new_min_num)
    }

    pub fn set_max_num(ctx: Context<SetGlobalPool>, new_max_num: u8) -> Result<()> {
        instructions::admin::set_max_num(ctx, new_max_num)
    }

    pub fn set_operation_authority(
        ctx: Context<SetAuthority>,
        new_operation_authority: Pubkey,
    ) -> Result<()> {
        instructions::admin::set_operation_authority(ctx, new_operation_authority)
    }

    pub fn set_finance_authority(
        ctx: Context<SetAuthority>,
        new_finance_authority: Pubkey,
    ) -> Result<()> {
        instructions::admin::set_finance_authority(ctx, new_finance_authority)
    }

    pub fn set_update_authority(
        ctx: Context<SetAuthority>,
        new_update_authority: Pubkey,
    ) -> Result<()> {
        instructions::admin::set_update_authority(ctx, new_update_authority)
    }
}
