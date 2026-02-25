use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;

use crate::errors::CoinflipError;
use crate::state::{Game, GameStatus, House};

#[derive(Accounts)]
pub struct CreateFlip<'info> {
    #[account(mut, has_one = vault)]
    pub house: Account<'info, House>,

    #[account(
        init,
        payer = player,
        space = 8 + 32 + 32 + 1 + 8 + 1 + 1 + 1 + 32 + 8,
        seeds = [b"game", house.key().as_ref(), game_authority.key().as_ref()],
        bump
    )]
    pub game: Account<'info, Game>,

    /// CHECK: vault PDA - holds SOL
    #[account(
        mut,
        seeds = [b"vault", house.key().as_ref()],
        bump = house.vault_bump
    )]
    pub vault: AccountInfo<'info>,

    #[account(mut)]
    pub player: Signer<'info>,

    pub game_authority: Signer<'info>,

    pub clock: Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateFlip>, side: u8, amount: u64) -> Result<()> {
    require!(side <= 1, CoinflipError::InvalidSide);
    require!(amount > 0, CoinflipError::InvalidAmount);

    let house = &ctx.accounts.house;
    let game = &mut ctx.accounts.game;

    game.player = ctx.accounts.player.key();
    game.game_authority = ctx.accounts.game_authority.key();
    game.side = side;
    game.amount = amount;
    game.status = GameStatus::Created;
    game.bump = ctx.bumps.game;
    game.house = house.key();
    game.created_at = ctx.accounts.clock.slot;

    let transfer_ix = system_instruction::transfer(
        &ctx.accounts.player.key(),
        &ctx.accounts.vault.key(),
        amount,
    );
    anchor_lang::solana_program::program::invoke(
        &transfer_ix,
        &[
            ctx.accounts.player.to_account_info(),
            ctx.accounts.vault.to_account_info(),
        ],
    )?;

    Ok(())
}
