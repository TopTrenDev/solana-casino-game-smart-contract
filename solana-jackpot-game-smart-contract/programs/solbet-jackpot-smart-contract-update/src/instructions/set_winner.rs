use crate::{ constants::*, errors::*, state::{ config::*, game_round::* }, utils::* };
use anchor_lang::prelude::*;
use orao_solana_vrf::RANDOMNESS_ACCOUNT_SEED;

#[derive(Accounts)]
#[instruction(force: [u8; 32], round: u64)]
pub struct SetWinner<'info> {
    #[account(mut, seeds = [CONFIG_SEED], bump)]
    pub config: Account<'info, Config>,

    #[account(mut, seeds = [ROUND_SEED, &round.to_le_bytes()], bump)]
    pub round_acc: Account<'info, GameRound>,

    /// CHECK: Randomness
    #[account(
        mut,
        seeds = [RANDOMNESS_ACCOUNT_SEED, &force],
        bump,
        seeds::program = orao_solana_vrf::ID
    )]
    pub random: AccountInfo<'info>,

    #[account(mut)]
    pub admin: Signer<'info>,
}

pub fn handler(ctx: Context<SetWinner>, force: [u8; 32], roundIndex: u64) -> Result<()> {
    let round = &mut ctx.accounts.round_acc;
    let config = &mut ctx.accounts.config;

    require!(ctx.accounts.admin.key() == config.admin, JackpotError::InvalidAuthority);
    require!(config.round_counter == roundIndex, JackpotError::InvalidRoundCounter);
    require!(!config.is_completed, JackpotError::RoundAlreadyCompleted);
    require!(round.total_amount > 0, JackpotError::InvalidAmount);

    // VRF randomness extraction - implementation removed for security
    let randomness = 0;
    if randomness == 0 {
        return err!(JackpotError::StillProcessing);
    }

    // Randomness processing - implementation details removed for security
    let rand = process_randomness(randomness, round.total_amount);

    round.select_winner(rand)?;

    Ok(())
}
