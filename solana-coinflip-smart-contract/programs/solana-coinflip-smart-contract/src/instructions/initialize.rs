use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;

use crate::errors::CoinflipError;
use crate::state::House;

#[derive(Accounts)]
pub struct InitializeHouse<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 2 + 1 + 1,
        seeds = [b"house", authority.key().as_ref()],
        bump
    )]
    pub house: Account<'info, House>,

    /// CHECK: vault PDA created as system account to hold SOL
    #[account(
        mut,
        seeds = [b"vault", house.key().as_ref()],
        bump
    )]
    pub vault: AccountInfo<'info>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeHouse>, fee_bps: u16) -> Result<()> {
    require!(fee_bps <= 10_000, CoinflipError::InvalidFee);

    let house = &mut ctx.accounts.house;
    house.authority = ctx.accounts.authority.key();
    house.fee_bps = fee_bps;
    house.bump = ctx.bumps.house;
    house.vault_bump = ctx.bumps.vault;

    let vault = &ctx.accounts.vault;
    let house_key = house.key();
    let seeds = &[b"vault", house_key.as_ref(), &[house.vault_bump]];
    let signer_seeds = &[&seeds[..]];
    let rent = Rent::get()?;
    let lamports = rent.minimum_balance(0);
    let create_ix = system_instruction::create_account(
        &ctx.accounts.authority.key(),
        &vault.key(),
        lamports,
        0,
        &ctx.accounts.system_program.key(),
    );
    anchor_lang::solana_program::program::invoke_signed(
        &create_ix,
        &[
            ctx.accounts.authority.to_account_info(),
            vault.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
        signer_seeds,
    )?;

    Ok(())
}
