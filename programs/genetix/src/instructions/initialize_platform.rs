use anchor_lang::prelude::*;
use crate::{state::*, errors::GameError, ANCHOR_DISCRIMINATOR};

#[derive(Accounts)]
pub struct InitializePlatform<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        space = ANCHOR_DISCRIMINATOR + Platform::INIT_SPACE,
        seeds = [b"platform"],
        bump
    )]
    pub platform: Account<'info, Platform>,

    #[account(
        init,
        payer = admin,
        space = ANCHOR_DISCRIMINATOR + PlatformTreasury::INIT_SPACE,
        seeds = [b"treasury"],
        bump
    )]
    pub treasury: Account<'info, PlatformTreasury>,

    pub system_program: Program<'info, System>,
}

pub fn initialize_platform(
    ctx: Context<InitializePlatform>,
    platform_fee_percentage: u16,
) -> Result<()> {
    require!(
        platform_fee_percentage <= 10000,
        GameError::InvalidFeePercentage
    );

    let platform = &mut ctx.accounts.platform;
    platform.admin = ctx.accounts.admin.key();
    platform.mon_token_mint = Pubkey::default();
    platform.soul_stone_mint = Pubkey::default();
    platform.soul_stone_price_lamports = 0;
    platform.platform_fee_percentage = platform_fee_percentage;
    platform.total_pokemon_minted = 0;
    platform.total_battles = 0;
    platform.bump = ctx.bumps.platform;

    let treasury = &mut ctx.accounts.treasury;
    treasury.total_fees_collected = 0;
    treasury.mon_token_vault = Pubkey::default();
    treasury.bump = ctx.bumps.treasury;

    msg!("Platform initialized with admin: {}", ctx.accounts.admin.key());
    msg!("Platform fee percentage: {}%", platform_fee_percentage as f64 / 100.0);

    Ok(())
}