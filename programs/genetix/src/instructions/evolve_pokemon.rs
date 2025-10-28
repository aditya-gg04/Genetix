use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        update_metadata_accounts_v2, mpl_token_metadata::types::DataV2,
        UpdateMetadataAccountsV2, Metadata,
    },
    token::{burn, Burn, Mint, Token, TokenAccount},
};
use crate::{state::*, errors::GameError};

#[derive(Accounts)]
pub struct EvolvePokemon<'info> {
    #[account(mut)]
    pub player: Signer<'info>,

    #[account(
        seeds = [b"platform"],
        bump = platform.bump
    )]
    pub platform: Account<'info, Platform>,

    /// Pokemon mint (plain Mint account)
    #[account(mut)]
    pub pokemon_mint: Account<'info, Mint>,

    /// Pokemon data PDA derived from the mint
    #[account(
        mut,
        seeds = [b"pokemon_data", pokemon_mint.key().as_ref()],
        bump = pokemon_data.bump
    )]
    pub pokemon_data: Account<'info, PokemonData>,

    /// CHECK: Metadata account for Pokemon NFT (validated at runtime)
    pub metadata_account: UncheckedAccount<'info>,

    // Soul Stone burn
    #[account(
        mut,
        address = platform.soul_stone_mint
    )]
    pub soul_stone_mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = soul_stone_mint,
        associated_token::authority = player
    )]
    pub player_soul_stone_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn evolve_pokemon(
    ctx: Context<EvolvePokemon>,
    new_metadata_uri: String,
    new_hp: u16,
    new_attack: u16,
    new_defense: u16,
    new_speed: u16,
) -> Result<()> {
    // Basic validations
    require!(new_metadata_uri.len() <= 200, GameError::UriTooLong);
    require!(
        new_hp > 0 && new_attack > 0 && new_defense > 0 && new_speed > 0,
        GameError::InvalidStats
    );
    require!(
        ctx.accounts.player_soul_stone_account.amount >= 1,
        GameError::NoSoulStone
    );

    // ----- Runtime ownership / consistency checks -----
    // Ensure pokemon_data.owner == player
    require_keys_eq!(
        ctx.accounts.pokemon_data.owner,
        ctx.accounts.player.key(),
        GameError::NotPokemonOwner
    );

    // Ensure pokemon_data.mint == provided pokemon_mint
    require_keys_eq!(
        ctx.accounts.pokemon_data.mint,
        ctx.accounts.pokemon_mint.key(),
        GameError::InvalidPokemonMint
    );

    // Optional but recommended: verify metadata PDA matches expected
    // PDA: pubkey::find_program_address(["metadata", token_metadata_program_id, mint], token_metadata_program_id)
    let binding = ctx.accounts.token_metadata_program.key();
    let pokemon_mint = ctx.accounts.pokemon_mint.key();
    let metadata_seeds: &[&[u8]] = &[
        b"metadata".as_ref(),
        binding.as_ref(),
        pokemon_mint.as_ref(),
    ];
    let (expected_metadata_pubkey, _bump) =
        Pubkey::find_program_address(metadata_seeds, &ctx.accounts.token_metadata_program.key());
    require_keys_eq!(
        ctx.accounts.metadata_account.key(),
        expected_metadata_pubkey,
        GameError::InvalidMetadataAccount
    );

    // ----- Burn Soul Stone -----
    burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.soul_stone_mint.to_account_info(),
                from: ctx.accounts.player_soul_stone_account.to_account_info(),
                authority: ctx.accounts.player.to_account_info(),
            },
        ),
        1,
    )?;

    msg!("Burned 1 Soul Stone for evolution");

    // ----- Update Metadata (CPI to token-metadata program) -----
    update_metadata_accounts_v2(
        CpiContext::new(
            ctx.accounts.token_metadata_program.to_account_info(),
            UpdateMetadataAccountsV2 {
                metadata: ctx.accounts.metadata_account.to_account_info(),
                update_authority: ctx.accounts.player.to_account_info(),
            },
        ),
        None, // new_update_authority
        Some(DataV2 {
            name: ctx.accounts.pokemon_data.name.clone(),
            symbol: "PKMN".to_string(),
            uri: new_metadata_uri.clone(),
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        }),
        None, // primary_sale_happened
        None, // is_mutable
    )?;

    msg!("Updated Pokemon metadata URI");

    // ----- Update On-chain PokemonData -----
    let pokemon_data = &mut ctx.accounts.pokemon_data;
    pokemon_data.current_metadata_uri = new_metadata_uri;
    pokemon_data.hp = new_hp;
    pokemon_data.attack = new_attack;
    pokemon_data.defense = new_defense;
    pokemon_data.speed = new_speed;
    pokemon_data.evolution_stage = pokemon_data
        .evolution_stage
        .checked_add(1)
        .ok_or(GameError::MathOverflow)?;
    pokemon_data.level = pokemon_data
        .level
        .checked_add(1)
        .ok_or(GameError::MathOverflow)?;

    msg!(
        "Pokemon evolved successfully to stage {}",
        pokemon_data.evolution_stage
    );

    Ok(())
}
