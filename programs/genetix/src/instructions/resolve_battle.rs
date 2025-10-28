use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer_checked, close_account, Mint, Token, TokenAccount, TransferChecked, CloseAccount},
};
use crate::{state::*, errors::GameError};

#[derive(Accounts)]
pub struct ResolveBattle<'info> {
    #[account(mut)]
    pub resolver: Signer<'info>, // Could be admin or oracle

    #[account(
        mut,
        seeds = [b"platform"],
        bump = platform.bump
    )]
    pub platform: Account<'info, Platform>,

    #[account(
        mut,
        seeds = [b"treasury"],
        bump = treasury.bump
    )]
    pub treasury: Account<'info, PlatformTreasury>,

    #[account(
        mut,
        close = player1,
        seeds = [b"battle", battle.battle_id.to_le_bytes().as_ref()],
        bump = battle.bump
    )]
    pub battle: Account<'info, Battle>,

    /// CHECK: Player 1 address
    #[account(mut)]
    pub player1: UncheckedAccount<'info>,

    /// CHECK: Player 2 address
    #[account(mut)]
    pub player2: UncheckedAccount<'info>,

    // MON token accounts
    #[account(
        mut,
        address = platform.mon_token_mint
    )]
    pub mon_token_mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mon_token_mint,
        associated_token::authority = battle
    )]
    pub battle_escrow: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = resolver,
        associated_token::mint = mon_token_mint,
        associated_token::authority = player1
    )]
    pub player1_mon_account: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = resolver,
        associated_token::mint = mon_token_mint,
        associated_token::authority = player2
    )]
    pub player2_mon_account: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = resolver,
        associated_token::mint = mon_token_mint,
        associated_token::authority = platform
    )]
    pub platform_fee_account: Account<'info, TokenAccount>,

    // Pokemon data for stats update
    #[account(
        mut,
        seeds = [b"pokemon_data", battle.player1_pokemon.key().as_ref()],
        bump = player1_pokemon_data.bump
    )]
    pub player1_pokemon_data: Account<'info, PokemonData>,

    #[account(
        mut,
        seeds = [b"pokemon_data", battle.player2_pokemon.unwrap().key().as_ref()],
        bump = player2_pokemon_data.bump
    )]
    pub player2_pokemon_data: Account<'info, PokemonData>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn resolve_battle(
    ctx: Context<ResolveBattle>,
    winner_is_player1: bool,
) -> Result<()> {
    let battle = &ctx.accounts.battle;

    require!(
        battle.status == BattleStatus::InProgress,
        GameError::InvalidBattleStatus
    );
    require!(
        battle.player2.is_some(),
        GameError::BattleNotReady
    );

    let total_pot = ctx.accounts.battle_escrow.amount;
    let platform_fee = battle.platform_fee_amount;
    let winner_amount = total_pot
        .checked_sub(platform_fee)
        .ok_or(GameError::MathOverflow)?;

    let binding = battle.battle_id.to_le_bytes();
    let battle_seeds = &[
        b"battle".as_ref(),
        binding.as_ref(),
        &[battle.bump],
    ];
    let signer_seeds = &[&battle_seeds[..]];

    // Transfer platform fee
    transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.battle_escrow.to_account_info(),
                mint: ctx.accounts.mon_token_mint.to_account_info(),
                to: ctx.accounts.platform_fee_account.to_account_info(),
                authority: ctx.accounts.battle.to_account_info(),
            },
            signer_seeds,
        ),
        platform_fee,
        ctx.accounts.mon_token_mint.decimals,
    )?;

    msg!("Platform fee collected: {}", platform_fee);

    // Transfer winnings to winner
    let winner_account = if winner_is_player1 {
        &ctx.accounts.player1_mon_account
    } else {
        &ctx.accounts.player2_mon_account
    };

    transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.battle_escrow.to_account_info(),
                mint: ctx.accounts.mon_token_mint.to_account_info(),
                to: winner_account.to_account_info(),
                authority: ctx.accounts.battle.to_account_info(),
            },
            signer_seeds,
        ),
        winner_amount,
        ctx.accounts.mon_token_mint.decimals,
    )?;

    msg!("Winner received: {} MON tokens", winner_amount);

    // Close escrow account
    close_account(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            CloseAccount {
                account: ctx.accounts.battle_escrow.to_account_info(),
                destination: ctx.accounts.resolver.to_account_info(),
                authority: ctx.accounts.battle.to_account_info(),
            },
            signer_seeds,
        ),
    )?;

    // Update Pokemon stats
    let current_time = Clock::get()?.unix_timestamp;
    
    if winner_is_player1 {
        ctx.accounts.player1_pokemon_data.battles_won = ctx.accounts.player1_pokemon_data.battles_won
            .checked_add(1)
            .ok_or(GameError::MathOverflow)?;
        ctx.accounts.player1_pokemon_data.last_battle_at = current_time;

        ctx.accounts.player2_pokemon_data.battles_lost = ctx.accounts.player2_pokemon_data.battles_lost
            .checked_add(1)
            .ok_or(GameError::MathOverflow)?;
        ctx.accounts.player2_pokemon_data.last_battle_at = current_time;
    } else {
        ctx.accounts.player2_pokemon_data.battles_won = ctx.accounts.player2_pokemon_data.battles_won
            .checked_add(1)
            .ok_or(GameError::MathOverflow)?;
        ctx.accounts.player2_pokemon_data.last_battle_at = current_time;

        ctx.accounts.player1_pokemon_data.battles_lost = ctx.accounts.player1_pokemon_data.battles_lost
            .checked_add(1)
            .ok_or(GameError::MathOverflow)?;
        ctx.accounts.player1_pokemon_data.last_battle_at = current_time;
    }

    // Update battle status
    let battle = &mut ctx.accounts.battle;
    battle.status = BattleStatus::Resolved;
    battle.winner = Some(if winner_is_player1 {
        battle.player1
    } else {
        battle.player2.unwrap()
    });
    battle.resolved_at = Some(current_time);

    // Update treasury
    ctx.accounts.treasury.total_fees_collected = ctx.accounts.treasury.total_fees_collected
        .checked_add(platform_fee)
        .ok_or(GameError::MathOverflow)?;

    msg!("Battle {} resolved. Winner: {}", 
        battle.battle_id,
        if winner_is_player1 { "Player 1" } else { "Player 2" }
    );

    Ok(())
}