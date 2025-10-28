use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};
use crate::{state::*, errors::GameError, ANCHOR_DISCRIMINATOR};

const BATTLE_STAKE_AMOUNT: u64 = 10_000_000_000; // 10 MON tokens (with 9 decimals)

#[derive(Accounts)]
#[instruction(battle_id: u64)]
pub struct CreateBattle<'info> {
    #[account(mut)]
    pub player1: Signer<'info>,

    #[account(
        mut,
        seeds = [b"platform"],
        bump = platform.bump
    )]
    pub platform: Account<'info, Platform>,

    #[account(
        init,
        payer = player1,
        space = ANCHOR_DISCRIMINATOR + Battle::INIT_SPACE,
        seeds = [b"battle", battle_id.to_le_bytes().as_ref()],
        bump
    )]
    pub battle: Account<'info, Battle>,

    // MON token staking
    #[account(
        mut,
        address = platform.mon_token_mint
    )]
    pub mon_token_mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mon_token_mint,
        associated_token::authority = player1
    )]
    pub player1_mon_account: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = player1,
        associated_token::mint = mon_token_mint,
        associated_token::authority = battle
    )]
    pub battle_escrow: Account<'info, TokenAccount>,

    // Pokemon verification
#[account(
    seeds = [b"pokemon_data", pokemon_mint.key().as_ref()],
    bump = pokemon_data.bump
)]
pub pokemon_data: Account<'info, PokemonData>,


    /// CHECK: Pokemon mint address
    pub pokemon_mint: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn create_battle(
    ctx: Context<CreateBattle>,
    battle_id: u64,
    pokemon_mint: Pubkey,
) -> Result<()> {
    require!(
        ctx.accounts.player1_mon_account.amount >= BATTLE_STAKE_AMOUNT,
        GameError::InsufficientMonTokens
    );
require_keys_eq!(
    ctx.accounts.pokemon_data.owner,
    ctx.accounts.player1.key(),
    GameError::NotPokemonOwner
);



    // Calculate platform fee
    let platform_fee_bps = ctx.accounts.platform.platform_fee_percentage;
    let platform_fee = (BATTLE_STAKE_AMOUNT as u128)
        .checked_mul(platform_fee_bps as u128)
        .and_then(|x| x.checked_div(10000))
        .and_then(|x| u64::try_from(x).ok())
        .ok_or(GameError::MathOverflow)?;

    // Transfer stake to escrow
    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.player1_mon_account.to_account_info(),
                to: ctx.accounts.battle_escrow.to_account_info(),
                authority: ctx.accounts.player1.to_account_info(),
            },
        ),
        BATTLE_STAKE_AMOUNT,
    )?;

    msg!("Player 1 staked {} MON tokens", BATTLE_STAKE_AMOUNT);

    // Initialize battle
    let battle = &mut ctx.accounts.battle;
    battle.battle_id = battle_id;
    battle.player1 = ctx.accounts.player1.key();
    battle.player2 = None;
    battle.player1_pokemon = pokemon_mint;
    battle.player2_pokemon = None;
    battle.stake_amount = BATTLE_STAKE_AMOUNT;
    battle.platform_fee_amount = platform_fee;
    battle.status = BattleStatus::WaitingForPlayer2;
    battle.winner = None;
    battle.created_at = Clock::get()?.unix_timestamp;
    battle.resolved_at = None;
    battle.bump = ctx.bumps.battle;

    // Update platform counter
    ctx.accounts.platform.total_battles = ctx.accounts.platform.total_battles
        .checked_add(1)
        .ok_or(GameError::MathOverflow)?;

    msg!("Battle {} created, waiting for opponent", battle_id);

    Ok(())
}