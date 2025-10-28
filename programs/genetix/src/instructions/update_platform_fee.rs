use anchor_lang::prelude::*;
use crate::{state::Platform, errors::GameError};

#[derive(Accounts)]
pub struct UpdatePlatformFee<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [b"platform"],
        bump = platform.bump,
        has_one = admin @ GameError::Unauthorized
    )]
    pub platform: Account<'info, Platform>,
}

pub fn update_platform_fee(
    ctx: Context<UpdatePlatformFee>,
    new_fee_percentage: u16,
) -> Result<()> {
    require!(
        new_fee_percentage <= 10000,
        GameError::InvalidFeePercentage
    );

    ctx.accounts.platform.platform_fee_percentage = new_fee_percentage;

    msg!("Platform fee updated to: {}%", new_fee_percentage as f64 / 100.0);

    Ok(())
}