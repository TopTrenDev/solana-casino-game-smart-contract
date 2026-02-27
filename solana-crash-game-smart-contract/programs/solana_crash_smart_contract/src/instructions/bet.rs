use anchor_lang::prelude::*;

use crate::state::{Bet, House, Round};
use crate::utils;

#[derive(Accounts)]
pub struct PlaceBet<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    pub house: Account<'info, House>,
    #[account(mut, has_one = house)]
    pub round: Account<'info, Round>,

    #[account(
        init,
        payer = player,
        space = 8 + Bet::INIT_SPACE,
        seeds = [crate::BET_SEED, round.key().as_ref(), player.key().as_ref()],
        bump
    )]
    pub bet: Account<'info, Bet>,

    #[account(
        mut,
        seeds = [crate::VAULT_SEED, house.key().as_ref()],
        bump = house.vault_bump
    )]
    /// CHECK
    pub vault: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ManualCashout<'info> {
    pub player: Signer<'info>,
    pub house: Account<'info, House>,
    pub round: Account<'info, Round>,
    #[account(
        mut,
        seeds = [crate::BET_SEED, round.key().as_ref(), player.key().as_ref()],
        bump = bet.bump,
        has_one = player,
        has_one = round
    )]
    pub bet: Account<'info, Bet>,
}

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    pub house: Account<'info, House>,
    pub round: Account<'info, Round>,
    #[account(
        mut,
        seeds = [crate::BET_SEED, round.key().as_ref(), player.key().as_ref()],
        bump = bet.bump,
        has_one = player,
        has_one = round
    )]
    pub bet: Account<'info, Bet>,

    #[account(
        mut,
        seeds = [crate::VAULT_SEED, house.key().as_ref()],
        bump = house.vault_bump
    )]
    /// CHECK
    pub vault: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub fn place_bet(
    ctx: Context<PlaceBet>,
    amount_lamports: u64,
    auto_cashout_x100: Option<u32>,
) -> Result<()> {
    let house = &ctx.accounts.house;
    let round = &ctx.accounts.round;
    require!(!house.paused, crate::errors::CrashError::Paused);
    require!(amount_lamports > 0, crate::errors::CrashError::InvalidAmount);
    require!(
        matches!(round.status, crate::state::RoundStatus::Betting),
        crate::errors::CrashError::InvalidRoundState
    );

    let now = Clock::get()?.unix_timestamp;
    require!(now < round.betting_ends_at, crate::errors::CrashError::BettingClosed);

    if let Some(x) = auto_cashout_x100 {
        require!(x >= 101, crate::errors::CrashError::InvalidMultiplier);
        require!(x <= 10_000, crate::errors::CrashError::InvalidMultiplier);
    }

    let bet = &mut ctx.accounts.bet;
    bet.round = round.key();
    bet.player = ctx.accounts.player.key();
    bet.amount = amount_lamports;
    bet.auto_cashout_x100 = auto_cashout_x100;
    bet.manual_cashout_x100 = None;
    bet.claimed = false;
    bet.bump = ctx.bumps.bet;

    let ix = anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.player.key(),
        &ctx.accounts.vault.key(),
        amount_lamports,
    );
    anchor_lang::solana_program::program::invoke(
        &ix,
        &[
            ctx.accounts.player.to_account_info(),
            ctx.accounts.vault.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;

    Ok(())
}

pub fn manual_cashout(ctx: Context<ManualCashout>, cashout_x100: u32) -> Result<()> {
    let round = &ctx.accounts.round;
    let bet = &mut ctx.accounts.bet;
    require!(
        matches!(round.status, crate::state::RoundStatus::Live),
        crate::errors::CrashError::InvalidRoundState
    );
    require!(!bet.claimed, crate::errors::CrashError::AlreadyClaimed);
    require!(
        cashout_x100 >= 101 && cashout_x100 <= 10_000,
        crate::errors::CrashError::InvalidMultiplier
    );

    bet.manual_cashout_x100 = Some(cashout_x100);
    Ok(())
}

pub fn claim(ctx: Context<Claim>) -> Result<()> {
    let round = &ctx.accounts.round;
    let bet = &mut ctx.accounts.bet;
    let house = &ctx.accounts.house;

    require!(
        matches!(round.status, crate::state::RoundStatus::Revealed),
        crate::errors::CrashError::RoundNotRevealed
    );
    require!(!bet.claimed, crate::errors::CrashError::AlreadyClaimed);

    let target_x100 = if let Some(m) = bet.manual_cashout_x100 {
        m
    } else if let Some(a) = bet.auto_cashout_x100 {
        a
    } else {
        return err!(crate::errors::CrashError::NoCashoutTarget);
    };

    require!(
        target_x100 <= round.crash_x100,
        crate::errors::CrashError::BetLost
    );

    let payout = utils::payout_amount(bet.amount, target_x100)?;
    require!(
        **ctx.accounts.vault.to_account_info().lamports.borrow() >= payout,
        crate::errors::CrashError::InsufficientVault
    );

    let house_key = house.key();
    let seeds = &[crate::VAULT_SEED, house_key.as_ref(), &[house.vault_bump]];
    let signer = &[&seeds[..]];

    let ix = anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.vault.key(),
        &ctx.accounts.player.key(),
        payout,
    );
    anchor_lang::solana_program::program::invoke_signed(
        &ix,
        &[
            ctx.accounts.vault.to_account_info(),
            ctx.accounts.player.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
        signer,
    )?;

    bet.claimed = true;
    Ok(())
}
