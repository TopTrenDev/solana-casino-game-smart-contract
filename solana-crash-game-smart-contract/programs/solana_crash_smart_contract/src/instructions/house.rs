use anchor_lang::prelude::*;

use crate::state::House;

#[derive(Accounts)]
pub struct InitializeHouse<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + House::INIT_SPACE,
        seeds = [crate::HOUSE_SEED, authority.key().as_ref()],
        bump
    )]
    pub house: Account<'info, House>,

    #[account(
        init,
        payer = authority,
        space = 0,
        seeds = [crate::VAULT_SEED, house.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetPaused<'info> {
    pub authority: Signer<'info>,
    #[account(
        mut,
        seeds = [crate::HOUSE_SEED, authority.key().as_ref()],
        bump = house.bump
    )]
    pub house: Account<'info, House>,
}

#[derive(Accounts)]
pub struct HouseDeposit<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        seeds = [crate::HOUSE_SEED, authority.key().as_ref()],
        bump = house.bump
    )]
    pub house: Account<'info, House>,
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
pub struct HouseWithdraw<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        seeds = [crate::HOUSE_SEED, authority.key().as_ref()],
        bump = house.bump
    )]
    pub house: Account<'info, House>,
    #[account(
        mut,
        seeds = [crate::VAULT_SEED, house.key().as_ref()],
        bump = house.vault_bump
    )]
    /// CHECK
    pub vault: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_house(ctx: Context<InitializeHouse>, edge_bps: u16) -> Result<()> {
    require!(edge_bps <= 2_000, crate::errors::CrashError::EdgeTooHigh);
    let house = &mut ctx.accounts.house;
    house.authority = ctx.accounts.authority.key();
    house.edge_bps = edge_bps;
    house.paused = false;
    house.bump = ctx.bumps.house;
    house.vault_bump = ctx.bumps.vault;
    Ok(())
}

pub fn set_paused(ctx: Context<SetPaused>, paused: bool) -> Result<()> {
    let house = &mut ctx.accounts.house;
    require_keys_eq!(
        house.authority,
        ctx.accounts.authority.key(),
        crate::errors::CrashError::Unauthorized
    );
    house.paused = paused;
    Ok(())
}

pub fn house_deposit(ctx: Context<HouseDeposit>, amount: u64) -> Result<()> {
    require!(amount > 0, crate::errors::CrashError::InvalidAmount);

    let ix = anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.authority.key(),
        &ctx.accounts.vault.key(),
        amount,
    );
    anchor_lang::solana_program::program::invoke(
        &ix,
        &[
            ctx.accounts.authority.to_account_info(),
            ctx.accounts.vault.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;
    Ok(())
}

pub fn house_withdraw(ctx: Context<HouseWithdraw>, amount: u64) -> Result<()> {
    require_keys_eq!(
        ctx.accounts.house.authority,
        ctx.accounts.authority.key(),
        crate::errors::CrashError::Unauthorized
    );
    require!(amount > 0, crate::errors::CrashError::InvalidAmount);
    require!(
        **ctx.accounts.vault.to_account_info().lamports.borrow() >= amount,
        crate::errors::CrashError::InsufficientVault
    );

    let house = &ctx.accounts.house;
    let house_key = house.key();
    let seeds = &[crate::VAULT_SEED, house_key.as_ref(), &[house.vault_bump]];
    let signer = &[&seeds[..]];

    let ix = anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.vault.key(),
        &ctx.accounts.authority.key(),
        amount,
    );
    anchor_lang::solana_program::program::invoke_signed(
        &ix,
        &[
            ctx.accounts.vault.to_account_info(),
            ctx.accounts.authority.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
        signer,
    )?;
    Ok(())
}
