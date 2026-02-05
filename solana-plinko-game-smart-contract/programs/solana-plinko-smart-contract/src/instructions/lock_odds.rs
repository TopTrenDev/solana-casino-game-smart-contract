use crate::account::*;
use crate::errors::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct LockOdds<'info> {
    #[account(
        mut,
        seeds = [b"plinko_status"],
        bump,
        constraint = plinko_status.is_owner(&authority.key()) @ PlinkoError::OnlyOwner
    )]
    pub plinko_status: Account<'info, PlinkoStatus>,

    pub authority: Signer<'info>,
}

pub fn handler(ctx: Context<LockOdds>) -> Result<()> {
    let plinko_status = &mut ctx.accounts.plinko_status;

    // Lock the odds
    plinko_status.odds_locked = true;

    msg!("Odds locked successfully");
    msg!("No further changes to payouts allowed");

    Ok(())
}
