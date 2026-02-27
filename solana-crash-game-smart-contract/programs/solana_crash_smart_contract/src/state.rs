use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct House {
    pub authority: Pubkey,
    pub edge_bps: u16,
    pub paused: bool,
    pub bump: u8,
    pub vault_bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Round {
    pub house: Pubkey,
    pub round_id: u64,
    pub commit_hash: [u8; 32],
    pub betting_ends_at: i64,
    pub round_ends_at: i64,
    pub status: RoundStatus,
    pub crash_x100: u32,
    pub revealed: bool,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Bet {
    pub round: Pubkey,
    pub player: Pubkey,
    pub amount: u64,
    pub auto_cashout_x100: Option<u32>,
    pub manual_cashout_x100: Option<u32>,
    pub claimed: bool,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum RoundStatus {
    Betting,
    Live,
    Revealed,
}
