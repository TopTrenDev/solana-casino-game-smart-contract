use crate::account::*;
use crate::errors::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + PlinkoStatus::LEN,
        seeds = [b"plinko_status"],
        bump
    )]
    pub plinko_status: Account<'info, PlinkoStatus>,

    #[account(init, payer = authority, space = 8 + House::LEN, seeds = [b"house"], bump)]
    pub house: Account<'info, House>,

    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK: This is a placeholder for the fee treasury account.
    pub fee_treasury: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<Initialize>,
    platform_fee: u64,
    min_buy_in: u64,
    max_balls: u8
) -> Result<()> {
    require!(platform_fee <= 300, PlinkoError::PlatformFeeTooHigh); // Max 3%
    require!(max_balls <= 60, PlinkoError::MaxBallsTooHigh); // Max 60 balls
    require!(min_buy_in > 0, PlinkoError::InvalidValue);

    let plinko_status = &mut ctx.accounts.plinko_status;
    let house = &mut ctx.accounts.house;

    plinko_status.owner = ctx.accounts.authority.key();
    plinko_status.platform_fee = platform_fee;
    plinko_status.fee_denominator = 10_000;
    plinko_status.payout_denominator = 100;
    plinko_status.min_buy_in = min_buy_in;
    plinko_status.max_balls = max_balls;
    plinko_status.odds_locked = false;
    plinko_status.paused = false;
    plinko_status.bucket_weights = Vec::new();
    plinko_status.payouts = Vec::new();
    plinko_status.total_games = 0;
    plinko_status.total_volume = 0;
    plinko_status.total_payouts = 0;
    plinko_status.fee_treasury = ctx.accounts.fee_treasury.key();
    plinko_status.house_account = house.key();

    house.owner = ctx.accounts.authority.key();
    house.balance = 0;
    house.maximum_payout = 100;
    house.total_payout = 0;
    house.withdrawals_pause = false;
    house.pending_request = 0;

    msg!("Plinko program initialized successfully");
    msg!("Owner: {}", plinko_status.owner);
    msg!("Platform Fee: {} basis points", plinko_status.platform_fee);
    msg!("Min Buy-in: {} lamports", plinko_status.min_buy_in);
    msg!("Max Balls: {}", plinko_status.max_balls);

    Ok(())
}
