use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};
use crate::{state::*, errors::GameError};

#[derive(Accounts)]
pub struct JoinBattle<'info> {
    #[account(mut)]
    pub player2: Signer<'info>,

    #[account(
        seeds = [b"platform"],
        bump = platform.bump
    )]
    pub platform: Account<'info, Platform>,

    #[account(
        mut,
        seeds = [b"battle", battle.battle_id.to_le_bytes().as_ref()],
        bump = battle.bump
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
        associated_token::authority = player2
    )]
    pub player2_mon_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mon_token_mint,
        associated_token::authority = battle
    )]
    pub battle_escrow: Account<'info, TokenAccount>,

    // Pokemon verification — remove has_one and check at runtime
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

pub fn join_battle(
    ctx: Context<JoinBattle>,
    pokemon_mint: Pubkey,
) -> Result<()> {
    let battle = &ctx.accounts.battle;

    require!(
        battle.status == BattleStatus::WaitingForPlayer2,
        GameError::InvalidBattleStatus
    );
    require!(
        battle.player1 != ctx.accounts.player2.key(),
        GameError::CannotJoinOwnBattle
    );
    require!(
        ctx.accounts.player2_mon_account.amount >= battle.stake_amount,
        GameError::InsufficientMonTokens
    );

    // --- Ownership check (replace compile-time `has_one` with this runtime check)
    require_keys_eq!(
        ctx.accounts.pokemon_data.owner,
        ctx.accounts.player2.key(),
        GameError::NotPokemonOwner
    );

    // --- Optional: ensure the pokemon_data actually corresponds to the supplied mint
    // (add a GameError variant like InvalidPokemonMint if you want to fail with a clear error)
    // require_keys_eq!(
    //     ctx.accounts.pokemon_data.mint,
    //     pokemon_mint,
    //     GameError::InvalidPokemonMint
    // );

    // Transfer stake to escrow
    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.player2_mon_account.to_account_info(),
                to: ctx.accounts.battle_escrow.to_account_info(),
                authority: ctx.accounts.player2.to_account_info(),
            },
        ),
        battle.stake_amount,
    )?;

    msg!("Player 2 staked {} MON tokens", battle.stake_amount);

    // Update battle
    let battle = &mut ctx.accounts.battle;
    battle.player2 = Some(ctx.accounts.player2.key());
    battle.player2_pokemon = Some(pokemon_mint);
    battle.status = BattleStatus::InProgress;

    msg!("Player 2 joined battle {}", battle.battle_id);
    msg!("Battle is now in progress!");

    Ok(())
}
