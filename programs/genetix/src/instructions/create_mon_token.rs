use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{
        create_metadata_accounts_v3, mpl_token_metadata::types::DataV2,
        CreateMetadataAccountsV3, Metadata,
    },
    token::{Mint, Token},
};
use crate::{state::Platform, errors::GameError, MON_TOKEN_DECIMALS};

#[derive(Accounts)]
pub struct CreateMonToken<'info> {
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
        mint::decimals = MON_TOKEN_DECIMALS,
        mint::authority = platform,
    )]
    pub mon_token_mint: Account<'info, Mint>,

    /// CHECK: Validate address by deriving pda
    #[account(
        mut,
        seeds = [
            b"metadata",
            token_metadata_program.key().as_ref(),
            mon_token_mint.key().as_ref()
        ],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    pub metadata_account: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn create_mon_token(
    ctx: Context<CreateMonToken>,
    token_name: String,
    token_symbol: String,
    token_uri: String,
) -> Result<()> {
    msg!("Creating MON token metadata");

    create_metadata_accounts_v3(
        CpiContext::new(
            ctx.accounts.token_metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                metadata: ctx.accounts.metadata_account.to_account_info(),
                mint: ctx.accounts.mon_token_mint.to_account_info(),
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

    // Update platform with MON token mint
    ctx.accounts.platform.mon_token_mint = ctx.accounts.mon_token_mint.key();

    msg!("MON token created successfully: {}", ctx.accounts.mon_token_mint.key());

    Ok(())
}