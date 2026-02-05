use crate::account::*;
use crate::utils::*;
use crate::errors::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct WithdrawFromVault<'info> {
    #[account(
        seeds = [b"plinko_status"],
        bump,
        constraint = plinko_status.is_owner(&authority.key()) @ PlinkoError::OnlyOwner
    )]
    pub plinko_status: Account<'info, PlinkoStatus>,

    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut, seeds = [b"vaultseed"], bump)]
    pub vault: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<WithdrawFromVault>, amount: u64) -> Result<()> {
    require!(ctx.accounts.vault.lamports() >= amount, PlinkoError::InsufficientFunds);
    let vault_bump = ctx.bumps.vault;

    sol_transfer_with_signer(
        ctx.accounts.vault.to_account_info(),
        ctx.accounts.authority.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
        &[&[b"vaultseed", &[vault_bump]]],
        amount
    )?;

    msg!("Withdrew {} lamports from Vault", amount);

    Ok(())
}
