use anchor_lang::prelude::*;

mod errors;
mod instructions;
mod state;
mod utils;

use instructions::{create_flip, initialize, resolve_flip};

declare_id!("6e75ejyxmXek3CB2xypobBaZMLywsWhZWCCybgvgexnt");

#[program]
pub mod coinflip {
    use super::*;

    pub fn initialize_house(ctx: Context<initialize::InitializeHouse>, fee_bps: u16) -> Result<()> {
        initialize::handler(ctx, fee_bps)
    }

    pub fn create_flip(
        ctx: Context<create_flip::CreateFlip>,
        side: u8,
        amount: u64,
    ) -> Result<()> {
        create_flip::handler(ctx, side, amount)
    }

    pub fn resolve_flip(ctx: Context<resolve_flip::ResolveFlip>) -> Result<()> {
        resolve_flip::handler(ctx)
    }
}
