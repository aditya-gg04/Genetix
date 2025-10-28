use anchor_lang::prelude::*;
use crate::{state::*, errors::GameError, ANCHOR_DISCRIMINATOR};

#[derive(Accounts)]
#[instruction(template_id: u64)]
pub struct AddPokemonTemplate<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        seeds = [b"platform"],
        bump = platform.bump,
        has_one = admin @ GameError::Unauthorized
    )]
    pub platform: Account<'info, Platform>,

    #[account(
        init,
        payer = admin,
        space = ANCHOR_DISCRIMINATOR + PokemonTemplate::INIT_SPACE,
        seeds = [b"template", template_id.to_le_bytes().as_ref()],
        bump
    )]
    pub pokemon_template: Account<'info, PokemonTemplate>,

    pub system_program: Program<'info, System>,
}

pub fn add_pokemon_template(
    ctx: Context<AddPokemonTemplate>,
    template_id: u64,
    name: String,
    base_uri: String,
    price_in_mon: u64,
    hp: u16,
    attack: u16,
    defense: u16,
    speed: u16,
) -> Result<()> {
    require!(name.len() <= 32, GameError::NameTooLong);
    require!(base_uri.len() <= 200, GameError::UriTooLong);
    require!(price_in_mon > 0, GameError::InvalidPrice);
    require!(
        hp > 0 && attack > 0 && defense > 0 && speed > 0,
        GameError::InvalidStats
    );

    let template = &mut ctx.accounts.pokemon_template;
    template.template_id = template_id;
    template.name = name;
    template.base_uri = base_uri;
    template.price_in_mon = price_in_mon;
    template.hp = hp;
    template.attack = attack;
    template.defense = defense;
    template.speed = speed;
    template.is_active = true;
    template.times_minted = 0;
    template.bump = ctx.bumps.pokemon_template;

    msg!("Pokémon template added: ID {}, Name: {}", template_id, template.name);

    Ok(())
}