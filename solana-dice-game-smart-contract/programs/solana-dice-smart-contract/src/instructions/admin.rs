use anchor_lang::prelude::*;
use solana_program::pubkey::Pubkey;

use crate::account::{SetAuthority, SetGlobalPool};
use crate::errors::GameError;

pub fn set_rtp(ctx: Context<SetGlobalPool>, new_rtp: u64) -> Result<()> {
    require!(new_rtp < 100, GameError::InvalidRtp);
    ctx.accounts.global_pool.rtp = new_rtp;
    Ok(())
}

pub fn set_max_win_amount(ctx: Context<SetGlobalPool>, new_max_win_amount: u64) -> Result<()> {
    ctx.accounts.global_pool.max_win_amount = new_max_win_amount;
    Ok(())
}

pub fn set_min_bet_amount(ctx: Context<SetGlobalPool>, new_min_bet_amount: u64) -> Result<()> {
    ctx.accounts.global_pool.min_bet_amount = new_min_bet_amount;
    Ok(())
}

pub fn set_min_num(ctx: Context<SetGlobalPool>, new_min_num: u8) -> Result<()> {
    require!(
        new_min_num <= ctx.accounts.global_pool.max_num,
        GameError::InvalidTargetNumber
    );
    ctx.accounts.global_pool.min_num = new_min_num;
    Ok(())
}

pub fn set_max_num(ctx: Context<SetGlobalPool>, new_max_num: u8) -> Result<()> {
    require!(
        new_max_num >= ctx.accounts.global_pool.min_num,
        GameError::InvalidTargetNumber
    );
    ctx.accounts.global_pool.max_num = new_max_num;
    Ok(())
}

pub fn set_operation_authority(
    ctx: Context<SetAuthority>,
    new_operation_authority: Pubkey,
) -> Result<()> {
    ctx.accounts.global_pool.operation_authority = new_operation_authority;
    Ok(())
}

pub fn set_finance_authority(
    ctx: Context<SetAuthority>,
    new_finance_authority: Pubkey,
) -> Result<()> {
    ctx.accounts.global_pool.finance_authority = new_finance_authority;
    Ok(())
}

pub fn set_update_authority(
    ctx: Context<SetAuthority>,
    new_update_authority: Pubkey,
) -> Result<()> {
    ctx.accounts.global_pool.update_authority = new_update_authority;
    Ok(())
}
