use anchor_lang::prelude::*;

use crate::account::Withdraw;
use crate::constants::VAULT_AUTHORITY_SEED;
use crate::errors::GameError;
use crate::utils::sol_transfer_with_signer;

pub fn handler(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    let global_authority = &ctx.accounts.global_authority;
    let financial_authority = &ctx.accounts.financial_admin;
    let casino_bump = ctx.bumps.casino_vault;
    let casino_vault = &ctx.accounts.casino_vault;

    require!(
        financial_authority.key() == global_authority.finance_authority,
        GameError::UnauthorizedFinanceAdmin
    );

    require!(
        casino_vault.lamports() > amount,
        GameError::InsufficientCasinoVault
    );

    sol_transfer_with_signer(
        ctx.accounts.casino_vault.to_account_info(),
        ctx.accounts.recipient.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
        &[&[VAULT_AUTHORITY_SEED.as_bytes(), &[casino_bump]]],
        amount,
    )?;

    let balance = ctx.accounts.casino_vault.to_account_info().lamports();
    msg!("Remaining balance: {:?}", balance);

    Ok(())
}
