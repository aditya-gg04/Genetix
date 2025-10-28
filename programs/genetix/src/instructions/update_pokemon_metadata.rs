use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{
        update_metadata_accounts_v2, mpl_token_metadata::types::DataV2,
        UpdateMetadataAccountsV2, Metadata,
    },
    token::Mint,
};
use crate::{state::PokemonData, errors::GameError};

#[derive(Accounts)]
pub struct UpdatePokemonMetadata<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        address = pokemon_data.mint
    )]
    pub pokemon_mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [b"pokemon_data", pokemon_mint.key().as_ref()],
        bump = pokemon_data.bump,
        has_one = owner @ GameError::NotPokemonOwner
    )]
    pub pokemon_data: Account<'info, PokemonData>,

    /// CHECK: Metadata account for Pokemon NFT
    #[account(
        mut,
        seeds = [
            b"metadata",
            token_metadata_program.key().as_ref(),
            pokemon_mint.key().as_ref()
        ],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    pub metadata_account: UncheckedAccount<'info>,

    pub token_metadata_program: Program<'info, Metadata>,
}

pub fn update_pokemon_metadata(
    ctx: Context<UpdatePokemonMetadata>,
    new_uri: String,
) -> Result<()> {
    require!(new_uri.len() <= 200, GameError::UriTooLong);
    require!(!new_uri.is_empty(), GameError::InvalidMetadataUri);

    // Update metadata URI
    update_metadata_accounts_v2(
        CpiContext::new(
            ctx.accounts.token_metadata_program.to_account_info(),
            UpdateMetadataAccountsV2 {
                metadata: ctx.accounts.metadata_account.to_account_info(),
                update_authority: ctx.accounts.owner.to_account_info(),
            },
        ),
        None, // new_update_authority
        Some(DataV2 {
            name: ctx.accounts.pokemon_data.name.clone(),
            symbol: "PKMN".to_string(),
            uri: new_uri.clone(),
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        }),
        None, // primary_sale_happened
        None, // is_mutable
    )?;

    // Update Pokemon data
    ctx.accounts.pokemon_data.current_metadata_uri = new_uri;

    msg!("Pokemon metadata URI updated successfully");

    Ok(())
}