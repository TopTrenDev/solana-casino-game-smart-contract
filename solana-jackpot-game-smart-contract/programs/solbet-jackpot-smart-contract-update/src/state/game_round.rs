// NOTE: Core winner selection algorithm implementation has been removed for security
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct DepositInfo {
    pub user: Pubkey,
    pub amount: u64,
}

#[account]
#[derive(Debug)]
pub struct GameRound {
    pub deposits: Vec<DepositInfo>,
    pub total_amount: u64,
    pub winner: Option<Pubkey>,
    pub winner_index: u64,
    pub winner_deposit_amount: u64,
    pub started_at: i64,
    pub ends_at: i64,
    pub is_expired: bool,
}

impl GameRound {
    // Account size calculation - implementation details removed for security
    pub const MAX_SIZE: usize = 0; // Placeholder - actual size calculation removed

    pub fn add_deposit(&mut self, user: Pubkey, amount: u64) {
        self.deposits.push(DepositInfo { user, amount });
        self.total_amount += amount;
    }

    // Core winner selection algorithm - implementation removed for security
    pub fn select_winner(&mut self, rand: u64) -> Result<()> {
        Ok(())
    }
}
