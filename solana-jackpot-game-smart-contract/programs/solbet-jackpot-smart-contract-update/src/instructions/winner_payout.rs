use crate::constants::*;
use crate::errors::*;
use crate::utils::*;
use crate::state::{ config::*, game_round::* };
use anchor_lang::prelude::*;
use anchor_lang::system_program::{ transfer, Transfer };

#[derive(Accounts)]
#[instruction(round: u64)]
pub struct WinnerPayout<'info> {
    #[account(mut, seeds = [CONFIG_SEED], bump)]
    pub config: Account<'info, Config>,

    #[account(mut, seeds = [ROUND_SEED, &round.to_le_bytes()], bump)]
    pub round_acc: Account<'info, GameRound>,

    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(mut, address = round_acc.winner.unwrap())]
    /// CHECK: Verified at runtime
    pub winner: AccountInfo<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut, seeds = [VAULT_SEED], bump)]
    pub vault: AccountInfo<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut, address = config.team_wallet)]
    pub team_wallet: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<WinnerPayout>, roundIndex: u64) -> Result<()> {
    let round = &mut ctx.accounts.round_acc;
    let config = &mut ctx.accounts.config;
    let vault_bump = ctx.bumps.vault;

    require!(ctx.accounts.admin.key() == config.admin, JackpotError::InvalidAuthority);
    require!(config.round_counter == roundIndex, JackpotError::InvalidRoundCounter);

    let winner_key = round.winner.ok_or(JackpotError::WinnerNotSet)?;

    require_keys_eq!(ctx.accounts.winner.key(), winner_key, JackpotError::NotWinner);

    let total = round.total_amount;
    require!(total > 0, JackpotError::InvalidAmount);

    // Fee calculation - implementation details removed for security
    let platform_fee = calculate_platform_fee(total, config.platform_fee);
    let reward = total.checked_sub(platform_fee).ok_or(JackpotError::InvalidAmount)?;

    // Transfer winner reward
    sol_transfer_with_signer(
        ctx.accounts.vault.to_account_info(),
        ctx.accounts.winner.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
        &[&[VAULT_SEED, &[vault_bump]]],
        reward
    )?;

    // Transfer platform fees to team wallet
    let vault_lamports = ctx.accounts.vault.to_account_info().lamports();
    let payout = vault_lamports;
    
    sol_transfer_with_signer(
        ctx.accounts.vault.to_account_info(),
        ctx.accounts.team_wallet.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
        &[&[VAULT_SEED, &[vault_bump]]],
        payout
    )?;

    config.is_completed = true;

    Ok(())
}
