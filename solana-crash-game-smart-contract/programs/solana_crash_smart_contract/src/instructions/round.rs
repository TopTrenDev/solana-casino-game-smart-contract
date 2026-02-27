use anchor_lang::prelude::*;
use anchor_lang::solana_program::keccak;

use crate::state::{Round, RoundStatus};
use crate::utils;

#[derive(Accounts)]
#[instruction(round_id: u64)]
pub struct StartRound<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        seeds = [crate::HOUSE_SEED, authority.key().as_ref()],
        bump = house.bump
    )]
    pub house: Account<'info, crate::state::House>,

    #[account(
        init,
        payer = authority,
        space = 8 + Round::INIT_SPACE,
        seeds = [crate::ROUND_SEED, house.key().as_ref(), &round_id.to_le_bytes()],
        bump
    )]
    pub round: Account<'info, Round>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CloseBetting<'info> {
    pub authority: Signer<'info>,
    #[account(
        seeds = [crate::HOUSE_SEED, authority.key().as_ref()],
        bump = house.bump
    )]
    pub house: Account<'info, crate::state::House>,
    #[account(mut, has_one = house)]
    pub round: Account<'info, Round>,
}

#[derive(Accounts)]
pub struct RevealRound<'info> {
    pub authority: Signer<'info>,
    #[account(
        seeds = [crate::HOUSE_SEED, authority.key().as_ref()],
        bump = house.bump
    )]
    pub house: Account<'info, crate::state::House>,
    #[account(mut, has_one = house)]
    pub round: Account<'info, Round>,
}

pub fn start_round(
    ctx: Context<StartRound>,
    round_id: u64,
    commit_hash: [u8; 32],
    betting_ends_at: i64,
    round_ends_at: i64,
) -> Result<()> {
    let house = &ctx.accounts.house;
    require!(!house.paused, crate::errors::CrashError::Paused);
    require!(
        house.authority == ctx.accounts.authority.key(),
        crate::errors::CrashError::Unauthorized
    );

    let now = Clock::get()?.unix_timestamp;
    require!(betting_ends_at > now, crate::errors::CrashError::InvalidTime);
    require!(
        round_ends_at > betting_ends_at,
        crate::errors::CrashError::InvalidTime
    );

    let round = &mut ctx.accounts.round;
    round.house = house.key();
    round.round_id = round_id;
    round.commit_hash = commit_hash;
    round.betting_ends_at = betting_ends_at;
    round.round_ends_at = round_ends_at;
    round.status = RoundStatus::Betting;
    round.crash_x100 = 0;
    round.revealed = false;
    round.bump = ctx.bumps.round;
    Ok(())
}

pub fn close_betting(ctx: Context<CloseBetting>) -> Result<()> {
    let round = &mut ctx.accounts.round;
    let now = Clock::get()?.unix_timestamp;
    require!(
        matches!(round.status, RoundStatus::Betting),
        crate::errors::CrashError::InvalidRoundState
    );
    require!(
        now >= round.betting_ends_at,
        crate::errors::CrashError::BettingStillOpen
    );

    round.status = RoundStatus::Live;
    Ok(())
}

pub fn reveal_and_settle_round(
    ctx: Context<RevealRound>,
    server_seed: [u8; 32],
    nonce: u64,
) -> Result<()> {
    let house = &ctx.accounts.house;
    let round = &mut ctx.accounts.round;
    require!(
        house.authority == ctx.accounts.authority.key(),
        crate::errors::CrashError::Unauthorized
    );
    require!(
        matches!(round.status, RoundStatus::Live),
        crate::errors::CrashError::InvalidRoundState
    );
    require!(!round.revealed, crate::errors::CrashError::AlreadyRevealed);

    let mut data = Vec::with_capacity(32 + 8 + 8);
    data.extend_from_slice(&server_seed);
    data.extend_from_slice(&nonce.to_le_bytes());
    data.extend_from_slice(&round.round_id.to_le_bytes());
    let digest = keccak::hash(&data).to_bytes();
    require!(digest == round.commit_hash, crate::errors::CrashError::BadReveal);

    let crash_x100 =
        utils::compute_crash_x100(server_seed, nonce, round.round_id, house.edge_bps)?;
    round.crash_x100 = crash_x100;
    round.revealed = true;
    round.status = RoundStatus::Revealed;

    Ok(())
}
