use crate::account::*;
use crate::errors::*;
use anchor_lang::prelude::*;
use orao_solana_vrf::program::OraoVrf;
use orao_solana_vrf::state::NetworkState;
use orao_solana_vrf::CONFIG_ACCOUNT_SEED;
use orao_solana_vrf::RANDOMNESS_ACCOUNT_SEED;

#[derive(Accounts)]
pub struct PlayGame<'info> {
    #[account(
        mut,
        seeds = [b"plinko_status"],
        bump,
        constraint = !plinko_status.paused @ PlinkoError::GamePaused,
        constraint = !plinko_status.bucket_weights.is_empty() @ PlinkoError::InvalidBucketIndex
    )]
    pub plinko_status: Account<'info, PlinkoStatus>,

    #[account(
        init,
        payer = player,
        space = 8 + Game::LEN,
        seeds = [b"game", game_id.to_le_bytes().as_ref()],
        bump
    )]
    pub game: Account<'info, Game>,

    #[account(
        mut,
        seeds = [b"house"],
        bump
    )]
    pub house: Account<'info, House>,

    #[account(
        init_if_needed,
        payer = player,
        space = 8 + UserStats::LEN,
        seeds = [b"user_stats", player.key().as_ref()],
        bump
    )]
    pub user_stats: Account<'info, UserStats>,

    #[account(mut)]
    pub player: Signer<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut, seeds = [b"vaultseed"], bump)]
    pub vault: AccountInfo<'info>,

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
    pub config: Account<'info, NetworkState>,

    pub vrf: Program<'info, OraoVrf>,

    /// CHECK: This account is used to verify the VRF program
    #[account(mut)]
    pub fee_treasury: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<PlayGame>,
    force: [u8; 32],
    game_id: u64,
    num_balls: u8,
    user_bet_amount: u64
) -> Result<()> {
    require!(
        num_balls > 0 && num_balls <= ctx.accounts.plinko_status.max_balls,
        PlinkoError::InvalidNumberOfBalls
    );
    require!(
        ctx.accounts.player.lamports() >= ctx.accounts.plinko_status.min_buy_in,
        PlinkoError::InvalidValue
    );
    require!(
        user_bet_amount > 0 &&
            user_bet_amount * (num_balls as u64) <= ctx.accounts.player.lamports() &&
            user_bet_amount >= ctx.accounts.plinko_status.min_buy_in,
        PlinkoError::InvalidBetAmount
    );

    let plinko_status = &mut ctx.accounts.plinko_status;
    let game = &mut ctx.accounts.game;
    let user_stats = &mut ctx.accounts.user_stats;
    let house = &mut ctx.accounts.house;
    let player = &mut ctx.accounts.player;
    let vault = &mut ctx.accounts.vault;

    let total_bet = user_bet_amount * (num_balls as u64);

    let platform_fee_amount =
        (total_bet * plinko_status.platform_fee) / plinko_status.fee_denominator;
    let amount_for_house = total_bet - platform_fee_amount;
    let bet_amount = (total_bet - platform_fee_amount) / (num_balls as u64);

    require!(bet_amount > 0, PlinkoError::InvalidBetAmount);

    anchor_lang::system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: player.to_account_info(),
                to: ctx.accounts.fee_treasury.to_account_info(),
            }
        ),
        platform_fee_amount
    )?;

    anchor_lang::system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: player.to_account_info(),
                to: vault.to_account_info(),
            }
        ),
        amount_for_house
    )?;

    msg!("Plinko {} game started", game_id);

    // Orao VRF call request
    let cpi_program = ctx.accounts.vrf.to_account_info();
    let cpi_accounts = orao_solana_vrf::cpi::accounts::RequestV2 {
        payer: player.to_account_info(),
        network_state: ctx.accounts.config.to_account_info(),
        treasury: ctx.accounts.treasury.to_account_info(),
        request: ctx.accounts.random.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    orao_solana_vrf::cpi::request_v2(cpi_ctx, force)?;

    plinko_status.force = force;
    plinko_status.status = Status::Processing;

    game.game_id = game_id;
    game.player = player.key();
    game.bet_amount = total_bet;
    game.amount_for_house = amount_for_house;
    game.num_balls = num_balls;
    game.bet_amount_per_ball = bet_amount;
    game.buckets = vec![0; num_balls as usize];
    game.payout = 0;
    game.has_ended = false;
    game.request_id = 0;
    game.created_at = Clock::get()?.unix_timestamp;
    game.ended_at = 0;

    if user_stats.user == Pubkey::default() {
        user_stats.user = player.key();
        user_stats.total_games = 0;
        user_stats.total_wagered = 0;
        user_stats.total_won = 0;
        user_stats.game_ids = Vec::new();
    }
    user_stats.total_games += 1;
    user_stats.total_wagered += total_bet;
    user_stats.game_ids.push(game_id);

    plinko_status.total_games += 1;
    plinko_status.total_volume += total_bet;

    let vault_lamports = vault.to_account_info().lamports();
    house.balance = vault_lamports;
    house.pending_request += 1;

    let request_id = plinko_status.generate_request_id(game_id, player.key());
    game.request_id = request_id;

    msg!("Game started successfully");
    msg!("Game ID: {}", game_id);
    msg!("Player: {}", player.key());
    msg!("Number of balls: {}", num_balls);
    msg!("Bet amount per ball: {} lamports", bet_amount);
    msg!("Total bet: {} lamports", total_bet);
    msg!("Platform fee: {} lamports", platform_fee_amount);
    msg!("Amount for house: {} lamports", amount_for_house);

    Ok(())
}
