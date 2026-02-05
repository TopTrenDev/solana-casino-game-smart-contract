use anchor_lang::prelude::*;
use crate::account::*;
use crate::errors::*;
use crate::misc::*;
use crate::utils::*;
use orao_solana_vrf::RANDOMNESS_ACCOUNT_SEED;

#[derive(Accounts)]
#[instruction(force: [u8; 32], game_id: u64, request_id: u64)]
pub struct FulFillRandomWords<'info> {
    #[account(
        mut, 
        seeds = [b"plinko_status"], 
        bump, 
    )]
    pub plinko_status: Account<'info, PlinkoStatus>,

    #[account(
        mut,
        seeds = [b"game", game_id.to_le_bytes().as_ref()],
        bump,
        constraint = game.request_id == request_id  @ PlinkoError::InvalidRequestId,
        constraint = !game.has_ended @ PlinkoError::GameAlreadyEnded
    )]
    pub game: Account<'info, Game>,

    #[account(
        mut,
        seeds = [b"house"],
        bump
    )]
    pub house: Account<'info, House>,

    #[account (mut, seeds = [b"user_stats", game.player.key().as_ref()], bump)]
    pub user_stats: Account<'info, UserStats>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut, seeds = [b"vaultseed"], bump)]
    pub vault: AccountInfo<'info>,

    /// CHECK: Randomness
    #[account(
        mut,
        seeds = [RANDOMNESS_ACCOUNT_SEED, &force],
        bump,
        seeds::program = orao_solana_vrf::ID
    )]
    pub random: AccountInfo<'info>,

    /// CHECK: This account is the player who played the game
    #[account(mut)]
    pub player: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<FulFillRandomWords>,
    force: [u8; 32],
    game_id: u64,
    request_id: u64
) -> Result<()> {
    let plinko_status = &mut ctx.accounts.plinko_status;
    let game = &mut ctx.accounts.game;
    let user_stats = &mut ctx.accounts.user_stats;
    let house = &mut ctx.accounts.house;
    let player = &mut ctx.accounts.player;
    let vault = &mut ctx.accounts.vault;
    let vault_bump = ctx.bumps.vault;

    let mut total_payout = 0u64;
    let mut buckets = Vec::new();
    let house_bump = ctx.bumps.house;

    let ball_amount: u8 = game.num_balls;
    let rand_acc = crate::misc::get_account_data(&ctx.accounts.random)?;

    let randomness = current_state(&rand_acc);
    msg!("Orao Random number: {}", randomness);
    if randomness == 0 {
        return err!(PlinkoError::StillProcessing);
    }

    let randoms: Vec<u16> = plinko_status.derive_many_randoms(randomness, ball_amount.into());

    // Fullfill each ball
    /*
       Skipping detailed comments for brevity
    */

    house.pending_request = house.pending_request.saturating_sub(1);
    game.has_ended = true;
    game.ended_at = Clock::get()?.unix_timestamp;
    plinko_status.status = Status::Finished;

    msg!("Game ended successfully");
    msg!("Game ID: {}", game.game_id);
    msg!("Player: {}", game.player);
    msg!("Total payout: {} lamports", total_payout);
    msg!("Buckets: {:?}", game.buckets);

    Ok(())
}
