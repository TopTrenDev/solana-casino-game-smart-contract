use anchor_lang::prelude::*;

use crate::account::PlayGame;
use crate::constants::VAULT_AUTHORITY_SEED;
use crate::errors::GameError;
use crate::state::GameStatus;
use crate::utils::sol_transfer_user;

pub fn handler(
    ctx: Context<PlayGame>,
    target_number: u8,
    is_under: bool,
    bet_amount: u64,
    _game_session_id: u64,
) -> Result<()> {
    let player_pool = &mut ctx.accounts.player_pool;
    let player = &ctx.accounts.owner;
    let global_authority = &ctx.accounts.global_authority;

    require!(
        global_authority.min_bet_amount <= bet_amount,
        GameError::InvalidBetAmount
    );

    require!(
        global_authority.min_num < target_number,
        GameError::InvalidTargetNumber
    );

    require!(
        global_authority.max_num > target_number,
        GameError::InvalidTargetNumber
    );

    let bet_amount_f64 = bet_amount as f64;
    let rtp_f64 = global_authority.rtp as f64;
    let rtp_ratio = rtp_f64 / 100.0;

    let new_balance = if is_under {
        let win_chance = target_number as f64 / 100.0;
        let multiplier = (1.0 / win_chance) * rtp_ratio;
        bet_amount_f64 * multiplier
    } else {
        let win_chance = (99 - target_number) as f64 / 100.0;
        let multiplier = (1.0 / win_chance) * rtp_ratio;
        bet_amount_f64 * multiplier
    };

    let net_gain = new_balance - bet_amount_f64;
    let net_gain_u64 = net_gain as u64;
    let max_win_amount_u64 = global_authority.max_win_amount;

    require!(
        net_gain_u64 < max_win_amount_u64,
        GameError::InvalidBetAmountMaxWinAmountViolation
    );

    require!(
        ctx.accounts.owner.to_account_info().lamports() > bet_amount,
        GameError::InsufficientUserBalance
    );

    require!(
        ctx.accounts.casino_vault.to_account_info().lamports() > bet_amount,
        GameError::InsufficientCasinoVault
    );

    sol_transfer_user(
        ctx.accounts.operator.to_account_info().clone(),
        player_pool.to_account_info().clone(),
        ctx.accounts.system_program.to_account_info().clone(),
        ctx.accounts.rent.minimum_balance(0),
    )?;

    sol_transfer_user(
        ctx.accounts.owner.to_account_info(),
        ctx.accounts.game_vault.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
        bet_amount,
    )?;

    player_pool.status = GameStatus::Active;
    player_pool.bet = bet_amount;
    player_pool.target_num = target_number;
    player_pool.is_under = is_under;
    player_pool.player = player.key();

    if is_under {
        msg!("User's choice is under {}", target_number);
    } else {
        msg!("User's choice is over {}", target_number);
    }

    Ok(())
}
