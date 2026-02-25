use anchor_lang::prelude::*;

use crate::errors::CoinflipError;
use crate::state::{FlipResult, Game, GameStatus, House};
use crate::utils::{compute_flip_outcome, compute_win_payout};

#[derive(Accounts)]
pub struct ResolveFlip<'info> {
    #[account(mut)]
    pub house: Account<'info, House>,

    #[account(
        mut,
        has_one = player,
        has_one = house
    )]
    pub game: Account<'info, Game>,

    /// CHECK: vault PDA
    #[account(
        mut,
        seeds = [b"vault", house.key().as_ref()],
        bump = house.vault_bump
    )]
    pub vault: AccountInfo<'info>,

    /// CHECK: player receives payout
    #[account(mut)]
    pub player: AccountInfo<'info>,

    pub clock: Sysvar<'info, Clock>,
}

pub fn handler(ctx: Context<ResolveFlip>) -> Result<()> {
    let game = &mut ctx.accounts.game;
    let house = &ctx.accounts.house;

    require!(game.status == GameStatus::Created, CoinflipError::AlreadyResolved);

    let resolve_slot = ctx.accounts.clock.slot;
    let outcome = compute_flip_outcome(game.key(), game.created_at, resolve_slot);
    let player_won = outcome == game.side;

    game.status = GameStatus::Resolved;
    game.result = if player_won {
        FlipResult::Win
    } else {
        FlipResult::Loss
    };

    if player_won {
        let payout = compute_win_payout(game.amount, house.fee_bps)
            .unwrap_or(0);

        **ctx.accounts.vault.to_account_info().try_borrow_mut_lamports()? -= payout;
        **ctx
            .accounts
            .player
            .to_account_info()
            .try_borrow_mut_lamports()? += payout;
    }

    Ok(())
}
