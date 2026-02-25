use anchor_lang::prelude::*;
use solana_program::pubkey::Pubkey;

#[account]
#[derive(Default)]
pub struct GlobalPool {
    pub super_admin: Pubkey,
    pub operation_authority: Pubkey,
    pub finance_authority: Pubkey,
    pub update_authority: Pubkey,
    pub rtp: u64,
    pub max_win_amount: u64,
    pub min_bet_amount: u64,
    pub min_num: u8,
    pub max_num: u8,
}

impl GlobalPool {
    pub const DATA_SIZE: usize = 32 + 32 + 32 + 32 + 8 + 8 + 8 + 1 + 1; // 154
}

#[derive(AnchorSerialize, AnchorDeserialize, Default, Clone, PartialEq)]
pub enum GameStatus {
    #[default]
    Active,
    Win,
    Lose,
}

#[account]
#[derive(Default)]
pub struct PlayerPool {
    pub bet: u64,           
    pub status: GameStatus, 
    pub is_under: bool,     
    pub target_num: u8,     
    pub player: Pubkey,     
}

impl PlayerPool {
    pub const DATA_SIZE: usize = 8 + 3 + 1 + 1 + 32; 
}
