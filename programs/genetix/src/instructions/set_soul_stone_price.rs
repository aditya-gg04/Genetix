use anchor_lang::prelude::*;
use crate::{state::*, errors::GameError};

#[derive(Accounts)]
pub struct SetSoulStonePrice<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [b"platform"],
        bump = platform.bump,
        has_one = admin @ GameError::Unauthorized
    )]
    pub platform: Account<'info, Platform>,

    #[account(
        mut,
        seeds = [b"soul_stone_config"],
        bump = soul_stone_config.bump
    )]
    pub soul_stone_config: Account<'info, SoulStoneConfig>,
}

pub fn set_soul_stone_price(
    ctx: Context<SetSoulStonePrice>,
    price_in_lamports: u64,
) -> Result<()> {
    require!(price_in_lamports > 0, GameError::InvalidPrice);

    ctx.accounts.soul_stone_config.price_in_lamports = price_in_lamports;
    ctx.accounts.platform.soul_stone_price_lamports = price_in_lamports;

    msg!("Soul Stone price set to {} lamports", price_in_lamports);

    Ok(())
}