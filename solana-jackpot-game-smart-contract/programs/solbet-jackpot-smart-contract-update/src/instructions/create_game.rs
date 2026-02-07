use anchor_lang::prelude::*;
use crate::state::{ config::Config, game_round::GameRound };
use crate::constants::*;
use crate::errors::*;
use orao_solana_vrf::program::OraoVrf;
use orao_solana_vrf::state::NetworkState;
use orao_solana_vrf::CONFIG_ACCOUNT_SEED;
use orao_solana_vrf::RANDOMNESS_ACCOUNT_SEED;

#[derive(Accounts)]
#[instruction(force: [u8; 32], round: u64)]
pub struct CreateGame<'info> {
    #[account(mut, seeds = [CONFIG_SEED], bump)]
    pub config: Account<'info, Config>,

    #[account(
        init,
        payer = admin,
        space = 8 + GameRound::MAX_SIZE, // adjust size as needed
        seeds = [ROUND_SEED, &round.to_le_bytes()],
        bump
    )]
    pub round_acc: Account<'info, GameRound>,

    #[account(mut)]
    pub admin: Signer<'info>,

    /// CHECK: Treasury
    #[account(mut)]
    pub treasury: AccountInfo<'info>,

    /// CHECK: Randomness
    #[account(
        mut,
        seeds = [RANDOMNESS_ACCOUNT_SEED, &force],
        bump,
        seeds::program = orao_solana_vrf::ID
    )]
    pub random: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [CONFIG_ACCOUNT_SEED],
        bump,
        seeds::program = orao_solana_vrf::ID
    )]
    pub network_config: Account<'info, NetworkState>,

    pub vrf: Program<'info, OraoVrf>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateGame>, force: [u8; 32], roundIndex: u64) -> Result<()> {
    let config = &mut ctx.accounts.config;
    let round = &mut ctx.accounts.round_acc;
    let clock = Clock::get()?;

    require!(ctx.accounts.admin.key() == config.admin, JackpotError::InvalidAuthority);
    if roundIndex <= config.round_counter {
        return err!(JackpotError::RoundAlreadyCompleted);
    }
    if roundIndex > config.round_counter + 1 {
        return err!(JackpotError::RoundAlreadyCompleted);
    }
    require!(roundIndex == config.round_counter + 1, JackpotError::InvalidRoundCounter);
    require!(config.is_completed == true, JackpotError::InvalidRoundCounter);

    let cpi_program = ctx.accounts.vrf.to_account_info();
    let cpi_accounts = orao_solana_vrf::cpi::accounts::RequestV2 {
        payer: ctx.accounts.admin.to_account_info(),
        network_state: ctx.accounts.network_config.to_account_info(),
        treasury: ctx.accounts.treasury.to_account_info(),
        request: ctx.accounts.random.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    orao_solana_vrf::cpi::request_v2(cpi_ctx, force)?;

    config.round_counter = roundIndex;
    config.is_completed = false;

    round.total_amount = 0;
    round.winner = None;
    round.started_at = 0;
    round.ends_at = 0;
    round.is_expired = false;
    round.deposits = vec![];

    Ok(())
}
