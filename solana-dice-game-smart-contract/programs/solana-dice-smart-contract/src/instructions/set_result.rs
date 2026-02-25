use anchor_lang::prelude::*;

use crate::account::SetResult;
use crate::constants::VAULT_AUTHORITY_SEED;
use crate::errors::GameError;
use crate::state::GameStatus;
use crate::utils::sol_transfer_with_signer;

pub fn handler(
    ctx: Context<SetResult>,
    is_win: bool,
    game_session_id: u64,
) -> Result<()> {
    let player_pool = &mut ctx.accounts.player_pool;
    let game_bump = ctx.bumps.game_vault;
    let casino_bump = ctx.bumps.casino_vault;
    let global_authority = &ctx.accounts.global_authority;
    let game_vault = &mut ctx.accounts.game_vault;
    let casino_vault = &mut ctx.accounts.casino_vault;
    let vault_balance = game_vault.lamports();

    let bet_amount_f64 = player_pool.bet as f64;
    let rtp_f64 = global_authority.rtp as f64;
    let rtp_ratio = rtp_f64 / 100.0;

    let win_balance = if player_pool.is_under {
        let win_chance = player_pool.target_num as f64 / 100.0;
        let multiplier = (1.0 / win_chance) * rtp_ratio;
        bet_amount_f64 * multiplier
    } else {
        let win_chance = (99 - player_pool.target_num) as f64 / 100.0;
        let multiplier = (1.0 / win_chance) * rtp_ratio;
        bet_amount_f64 * multiplier
    };

    if is_win {
        let win_balance_u64 = win_balance as u64;
        let casino_top_up = win_balance_u64
            .checked_sub(vault_balance)
            .ok_or(GameError::InvalidBetAmount)?;
        if casino_top_up > 0 {
            sol_transfer_with_signer(
                casino_vault.to_account_info(),
                game_vault.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
                &[&[VAULT_AUTHORITY_SEED.as_bytes(), &[casino_bump]]],
                casino_top_up,
            )?;
        }

        sol_transfer_with_signer(
            game_vault.to_account_info(),
            ctx.accounts.owner.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            &[&[
                ctx.accounts.owner.key().as_ref(),
                VAULT_AUTHORITY_SEED.as_bytes(),
                &game_session_id.to_be_bytes()[..],
                &[game_bump],
            ]],
            win_balance_u64,
        )?;

        player_pool.status = GameStatus::Win;
    } else {
        sol_transfer_with_signer(
            game_vault.to_account_info(),
            casino_vault.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            &[&[
                ctx.accounts.owner.key().as_ref(),
                VAULT_AUTHORITY_SEED.as_bytes(),
                &game_session_id.to_be_bytes()[..],
                &[game_bump],
            ]],
            vault_balance,
        )?;

        player_pool.status = GameStatus::Lose;
    }

    Ok(())
}
