use anchor_lang::prelude::Pubkey;
use anchor_lang::solana_program::hash::Hash;

#[inline]
pub fn compute_flip_outcome(game_key: Pubkey, created_at_slot: u64, resolve_slot: u64) -> u8 {
    let mut data = game_key.to_bytes().to_vec();
    data.extend_from_slice(&created_at_slot.to_le_bytes());
    data.extend_from_slice(&resolve_slot.to_le_bytes());
    let h = Hash::hash(&data);
    let seed = u64::from_le_bytes(h.to_bytes()[..8].try_into().unwrap());
    (seed % 2) as u8
}

#[inline]
pub fn compute_win_payout(amount: u64, fee_bps: u16) -> Option<u64> {
    let double = amount.checked_mul(2)?;
    let fee = double
        .checked_mul(fee_bps as u64)?
        .checked_div(10_000)?;
    double.checked_sub(fee)
}
