use anchor_lang::prelude::*;

#[account]
pub struct House {
    pub authority: Pubkey,
    pub fee_bps: u16,
    pub bump: u8,
    pub vault_bump: u8,
}

#[account]
pub struct Game {
    pub player: Pubkey,
    pub game_authority: Pubkey,
    pub side: u8,
    pub amount: u64,
    pub status: GameStatus,
    pub result: FlipResult,
    pub bump: u8,
    pub house: Pubkey,
    pub created_at: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum GameStatus {
    Created,
    Resolved,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Default)]
pub enum FlipResult {
    #[default]
    Pending,
    Win,
    Loss,
}
