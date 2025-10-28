use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{
        create_metadata_accounts_v3, mpl_token_metadata::types::DataV2,
        CreateMetadataAccountsV3, Metadata,
    },
    token::{Mint, Token},
};
use crate::{state::*, errors::GameError, ANCHOR_DISCRIMINATOR, SOUL_STONE_DECIMALS};

#[derive(Accounts)]
pub struct CreateSoulStoneToken<'info> {
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
        init,
        payer = admin,
        mint::decimals = SOUL_STONE_DECIMALS,
        mint::authority = platform,
    )]
    pub soul_stone_mint: Account<'info, Mint>,

    /// CHECK: Validate address by deriving pda
    #[account(
        mut,
        seeds = [
            b"metadata",
            token_metadata_program.key().as_ref(),
            soul_stone_mint.key().as_ref()
        ],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    pub metadata_account: UncheckedAccount<'info>,

    #[account(
        init,
        payer = admin,
        space = ANCHOR_DISCRIMINATOR + SoulStoneConfig::INIT_SPACE,
        seeds = [b"soul_stone_config"],
        bump
    )]
    pub soul_stone_config: Account<'info, SoulStoneConfig>,

    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn create_soul_stone_token(
    ctx: Context<CreateSoulStoneToken>,
    token_name: String,
    token_symbol: String,
    token_uri: String,
) -> Result<()> {
    msg!("Creating Soul Stone token metadata");

    create_metadata_accounts_v3(
        CpiContext::new(
            ctx.accounts.token_metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                metadata: ctx.accounts.metadata_account.to_account_info(),
                mint: ctx.accounts.soul_stone_mint.to_account_info(),
                mint_authority: ctx.accounts.platform.to_account_info(),
                update_authority: ctx.accounts.platform.to_account_info(),
                payer: ctx.accounts.admin.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
        ),
        DataV2 {
            name: token_name,
            symbol: token_symbol,
            uri: token_uri,
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        },
        true, // Is mutable
        true, // Update authority is signer
        None, // Collection details
    )?;

    // Update platform with Soul Stone mint
    ctx.accounts.platform.soul_stone_mint = ctx.accounts.soul_stone_mint.key();

    // Initialize Soul Stone config
    let config = &mut ctx.accounts.soul_stone_config;
    config.mint = ctx.accounts.soul_stone_mint.key();
    config.price_in_lamports = 0;
    config.total_minted = 0;
    config.bump = ctx.bumps.soul_stone_config;

    msg!("Soul Stone token created successfully: {}", ctx.accounts.soul_stone_mint.key());

    Ok(())
}