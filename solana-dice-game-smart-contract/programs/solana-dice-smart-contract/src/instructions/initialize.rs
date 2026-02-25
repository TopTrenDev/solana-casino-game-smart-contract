use anchor_lang::prelude::*;
use solana_program::pubkey::Pubkey;

use crate::account::Initialize;
use crate::constants::*;
use crate::state::GlobalPool;

pub fn handler(
    ctx: Context<Initialize>,
    operate_admin: Pubkey,
    financial_admin: Pubkey,
    update_admin: Pubkey,
) -> Result<()> {
    let global_authority = &mut ctx.accounts.global_authority;

    global_authority.super_admin = ctx.accounts.admin.key();
    global_authority.operation_authority = operate_admin;
    global_authority.finance_authority = financial_admin;
    global_authority.update_authority = update_admin;
    global_authority.rtp = RTP;
    global_authority.max_win_amount = MAX_WIN_AMOUNT;
    global_authority.min_bet_amount = MIN_BET_AMOUNT;
    global_authority.min_num = MIN_NUMBER;
    global_authority.max_num = MAX_NUMBER;

    Ok(())
}
