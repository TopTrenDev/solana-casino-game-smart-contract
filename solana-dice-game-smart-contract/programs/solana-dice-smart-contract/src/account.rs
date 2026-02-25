use anchor_lang::prelude::*;

use crate::constants::{GLOBAL_AUTHORITY_SEED, PLAYER_POOL_SEED, VAULT_AUTHORITY_SEED};
use crate::state::{GlobalPool, PlayerPool};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        space = 8 + GlobalPool::DATA_SIZE,
        seeds = [GLOBAL_AUTHORITY_SEED.as_bytes()],
        bump,
        payer = admin
    )]
    pub global_authority: Account<'info, GlobalPool>,

    #[account(
        mut,
        seeds = [VAULT_AUTHORITY_SEED.as_bytes()],
        bump,
    )]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub casino_vault: AccountInfo<'info>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct InitPlayGameParams {
    pub target_number: u8,
    pub is_under: bool,
    pub bet_amount: u64,
    pub game_session_id: u64,
}

#[derive(Accounts)]
#[instruction(params: InitPlayGameParams)]
pub struct PlayGame<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        address = global_authority.operation_authority
    )]
    pub operator: Signer<'info>,

    #[account(
        init,
        space = 8 + PlayerPool::DATA_SIZE,
        seeds = [&owner.key().as_ref(), PLAYER_POOL_SEED.as_bytes(), &params.game_session_id.to_be_bytes()[..]],
        bump,
        payer = operator
    )]
    pub player_pool: Account<'info, PlayerPool>,

    #[account(
        mut,
        seeds = [GLOBAL_AUTHORITY_SEED.as_bytes()],
        bump,
    )]
    pub global_authority: Box<Account<'info, GlobalPool>>,

    #[account(
        mut,
        seeds = [VAULT_AUTHORITY_SEED.as_bytes()],
        bump,
    )]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub casino_vault: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [&owner.key().as_ref(), VAULT_AUTHORITY_SEED.as_bytes(), &params.game_session_id.to_be_bytes()[..]],
        bump,
    )]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub game_vault: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct SetResultParams {
    pub is_win: bool,
    pub game_session_id: u64,
}

#[derive(Accounts)]
#[instruction(params: SetResultParams)]
pub struct SetResult<'info> {
    #[account(
        mut,
        address = global_authority.operation_authority
    )]
    pub operator: Signer<'info>,

    /// CHECK:
    #[account(mut)]
    pub owner: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [GLOBAL_AUTHORITY_SEED.as_bytes()],
        bump,
    )]
    pub global_authority: Box<Account<'info, GlobalPool>>,

    #[account(
        mut,
        close = operator,
        seeds = [&owner.key().as_ref(), PLAYER_POOL_SEED.as_bytes(), &params.game_session_id.to_be_bytes()[..]],
        bump
    )]
    pub player_pool: Account<'info, PlayerPool>,

    #[account(
        mut,
        seeds = [&owner.key().as_ref(), VAULT_AUTHORITY_SEED.as_bytes(), &params.game_session_id.to_be_bytes()[..]],
        bump,
    )]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub game_vault: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [VAULT_AUTHORITY_SEED.as_bytes()],
        bump,
    )]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub casino_vault: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(
        mut,
        address = global_authority.finance_authority
    )]
    pub financial_admin: Signer<'info>,

    /// CHECK:
    #[account(mut)]
    pub recipient: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [GLOBAL_AUTHORITY_SEED.as_bytes()],
        bump,
    )]
    pub global_authority: Box<Account<'info, GlobalPool>>,

    #[account(
        mut,
        seeds = [VAULT_AUTHORITY_SEED.as_bytes()],
        bump,
    )]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub casino_vault: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetGlobalPool<'info> {
    #[account(address = global_pool.update_authority)]
    pub admin: Signer<'info>,

    #[account(mut)]
    pub global_pool: Account<'info, GlobalPool>,
}

#[derive(Accounts)]
pub struct SetAuthority<'info> {
    #[account(address = global_pool.super_admin)]
    pub admin: Signer<'info>,

    #[account(mut)]
    pub global_pool: Account<'info, GlobalPool>,
}
