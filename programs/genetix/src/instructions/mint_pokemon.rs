use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        create_master_edition_v3, create_metadata_accounts_v3,
        mpl_token_metadata::types::DataV2, CreateMasterEditionV3, CreateMetadataAccountsV3,
        Metadata,
    },
    token::{mint_to, transfer, Mint, MintTo, Token, TokenAccount, Transfer},
};
use crate::{state::*, errors::GameError, ANCHOR_DISCRIMINATOR};

#[derive(Accounts)]
#[instruction(template_id: u64)]
pub struct MintPokemon<'info> {
    #[account(mut)]
    pub player: Signer<'info>,

    #[account(
        mut,
        seeds = [b"platform"],
        bump = platform.bump
    )]
    pub platform: Account<'info, Platform>,

    #[account(
        mut,
        seeds = [b"template", template_id.to_le_bytes().as_ref()],
        bump = pokemon_template.bump
    )]
    pub pokemon_template: Account<'info, PokemonTemplate>,

    // MON token payment
    #[account(mut)]
    pub mon_token_mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mon_token_mint,
        associated_token::authority = player
    )]
    pub player_mon_account: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = player,
        associated_token::mint = mon_token_mint,
        associated_token::authority = platform
    )]
    pub platform_mon_account: Account<'info, TokenAccount>,

    // Pokemon NFT
    #[account(
        init,
        payer = player,
        mint::decimals = 0,
        mint::authority = player,
        mint::freeze_authority = player,
    )]
    pub pokemon_mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = player,
        associated_token::mint = pokemon_mint,
        associated_token::authority = player
    )]
    pub player_pokemon_account: Account<'info, TokenAccount>,

    /// CHECK: Validate address by deriving pda
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

    /// CHECK: Validate address by deriving pda
    #[account(
        mut,
        seeds = [
            b"metadata",
            token_metadata_program.key().as_ref(),
            pokemon_mint.key().as_ref(),
            b"edition"
        ],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    pub edition_account: UncheckedAccount<'info>,

    #[account(
        init,
        payer = player,
        space = ANCHOR_DISCRIMINATOR + PokemonData::INIT_SPACE,
        seeds = [b"pokemon_data", pokemon_mint.key().as_ref()],
        bump
    )]
    pub pokemon_data: Account<'info, PokemonData>,

    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn mint_pokemon(
    ctx: Context<MintPokemon>,
    template_id: u64,
) -> Result<()> {
    let template = &ctx.accounts.pokemon_template;
    
    require!(template.is_active, GameError::TemplateNotFound);
    require!(
        ctx.accounts.player_mon_account.amount >= template.price_in_mon,
        GameError::InsufficientMonTokens
    );

    // Transfer MON tokens to platform
    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.player_mon_account.to_account_info(),
                to: ctx.accounts.platform_mon_account.to_account_info(),
                authority: ctx.accounts.player.to_account_info(),
            },
        ),
        template.price_in_mon,
    )?;

    msg!("Paid {} MON tokens for Pokemon", template.price_in_mon);

    // Mint NFT to player
    mint_to(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.pokemon_mint.to_account_info(),
                to: ctx.accounts.player_pokemon_account.to_account_info(),
                authority: ctx.accounts.player.to_account_info(),
            },
        ),
        1,
    )?;

    msg!("Minted Pokemon NFT");

    // Create metadata
    create_metadata_accounts_v3(
        CpiContext::new(
            ctx.accounts.token_metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                metadata: ctx.accounts.metadata_account.to_account_info(),
                mint: ctx.accounts.pokemon_mint.to_account_info(),
                mint_authority: ctx.accounts.player.to_account_info(),
                update_authority: ctx.accounts.player.to_account_info(),
                payer: ctx.accounts.player.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
        ),
        DataV2 {
            name: template.name.clone(),
            symbol: "PKMN".to_string(),
            uri: template.base_uri.clone(),
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        },
        true, // Is mutable
        true, // Update authority is signer
        None, // Collection details
    )?;

    msg!("Created Pokemon metadata");

    // Create master edition
    create_master_edition_v3(
        CpiContext::new(
            ctx.accounts.token_metadata_program.to_account_info(),
            CreateMasterEditionV3 {
                edition: ctx.accounts.edition_account.to_account_info(),
                mint: ctx.accounts.pokemon_mint.to_account_info(),
                update_authority: ctx.accounts.player.to_account_info(),
                mint_authority: ctx.accounts.player.to_account_info(),
                payer: ctx.accounts.player.to_account_info(),
                metadata: ctx.accounts.metadata_account.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
        ),
        None, // Max Supply
    )?;

    msg!("Created master edition");

    // Initialize Pokemon data
    let pokemon_data = &mut ctx.accounts.pokemon_data;
    pokemon_data.owner = ctx.accounts.player.key();
    pokemon_data.mint = ctx.accounts.pokemon_mint.key();
    pokemon_data.template_id = template_id;
    pokemon_data.name = template.name.clone();
    pokemon_data.current_metadata_uri = template.base_uri.clone();
    pokemon_data.hp = template.hp;
    pokemon_data.attack = template.attack;
    pokemon_data.defense = template.defense;
    pokemon_data.speed = template.speed;
    pokemon_data.level = 1;
    pokemon_data.evolution_stage = 0;
    pokemon_data.battles_won = 0;
    pokemon_data.battles_lost = 0;
    pokemon_data.created_at = Clock::get()?.unix_timestamp;
    pokemon_data.last_battle_at = 0;
    pokemon_data.bump = ctx.bumps.pokemon_data;

    // Update counters
    ctx.accounts.pokemon_template.times_minted = ctx.accounts.pokemon_template.times_minted
        .checked_add(1)
        .ok_or(GameError::MathOverflow)?;
    
    ctx.accounts.platform.total_pokemon_minted = ctx.accounts.platform.total_pokemon_minted
        .checked_add(1)
        .ok_or(GameError::MathOverflow)?;

    msg!("Pokemon minted successfully: {}", ctx.accounts.pokemon_mint.key());

    Ok(())
}