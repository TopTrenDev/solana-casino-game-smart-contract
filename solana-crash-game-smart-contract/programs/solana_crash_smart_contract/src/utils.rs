use anchor_lang::prelude::*;
use anchor_lang::solana_program::keccak;

use crate::errors::CrashError;

pub fn payout_amount(amount: u64, x100: u32) -> Result<u64> {
    let amt = amount as u128;
    let mul = x100 as u128;
    let out = amt
        .checked_mul(mul)
        .ok_or(CrashError::MathOverflow)?
        .checked_div(100)
        .ok_or(CrashError::MathOverflow)?;
    u64::try_from(out).map_err(|_| error!(CrashError::MathOverflow))
}

pub fn compute_crash_x100(
    server_seed: [u8; 32],
    nonce: u64,
    round_id: u64,
    edge_bps: u16,
) -> Result<u32> {
    let mut data = Vec::with_capacity(32 + 8 + 8 + 5);
    data.extend_from_slice(&server_seed);
    data.extend_from_slice(&nonce.to_le_bytes());
    data.extend_from_slice(&round_id.to_le_bytes());
    data.extend_from_slice(b"crash");
    let h = keccak::hash(&data).to_bytes();

    let r = u64::from_le_bytes(h[0..8].try_into().unwrap());
    let denom_max: u128 = u64::MAX as u128 + 1;
    let u_scaled = (r as u128)
        .checked_mul(1_000_000_000_000u128)
        .ok_or(CrashError::MathOverflow)?
        .checked_div(denom_max)
        .ok_or(CrashError::MathOverflow)?;

    let one_minus_u = 1_000_000_000_000u128
        .checked_sub(u_scaled)
        .ok_or(CrashError::MathOverflow)?
        .max(1);

    let edge_numerator = (10_000u128)
        .checked_sub(edge_bps as u128)
        .ok_or(CrashError::MathOverflow)?;

    let numerator = edge_numerator
        .checked_mul(100u128)
        .ok_or(CrashError::MathOverflow)?
        .checked_mul(1_000_000_000_000u128)
        .ok_or(CrashError::MathOverflow)?
        .checked_div(10_000u128)
        .ok_or(CrashError::MathOverflow)?;

    let crash = numerator
        .checked_div(one_minus_u)
        .ok_or(CrashError::MathOverflow)?;

    let crash_x100 = crash.max(100).min(100_000);
    u32::try_from(crash_x100).map_err(|_| error!(CrashError::MathOverflow))
}
