use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};
use crate::{state::*, errors::GameError};

#[derive(Accounts)]
pub struct MintSoulStone<'info> {
    #[account(mut)]
    pub player: Signer<'info>,

    #[account(
        seeds = [b"platform"],
        bump = platform.bump
    )]
    pub platform: Account<'info, Platform>,

    #[account(
        mut,
        seeds = [b"soul_stone_config"],
        bump = soul_stone_config.bump
    )]
    pub soul_stone_config: Account<'info, SoulStoneConfig>,

    #[account(
        mut,
        address = platform.soul_stone_mint
    )]
    pub soul_stone_mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = player,
        associated_token::mint = soul_stone_mint,
        associated_token::authority = player
    )]
    pub player_soul_stone_account: Account<'info, TokenAccount>,

    /// CHECK: Admin account to receive SOL payment
    #[account(
        mut,
        address = platform.admin
    )]
    pub admin: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn mint_soul_stone(ctx: Context<MintSoulStone>) -> Result<()> {
    let price = ctx.accounts.soul_stone_config.price_in_lamports;
    
    require!(price > 0, GameError::InvalidPrice);
    require!(
        ctx.accounts.player.lamports() >= price,
        GameError::InsufficientSol
    );

    // Transfer SOL to admin
    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.player.to_account_info(),
                to: ctx.accounts.admin.to_account_info(),
            },
        ),
        price,
    )?;

    msg!("Paid {} lamports for Soul Stone", price);

    // Mint Soul Stone to player
    let platform_seeds = &[
        b"platform".as_ref(),
        &[ctx.accounts.platform.bump],
    ];
    let signer_seeds = &[&platform_seeds[..]];

    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.soul_stone_mint.to_account_info(),
                to: ctx.accounts.player_soul_stone_account.to_account_info(),
                authority: ctx.accounts.platform.to_account_info(),
            },
            signer_seeds,
        ),
        1,
    )?;

    // Update counter
    ctx.accounts.soul_stone_config.total_minted = ctx.accounts.soul_stone_config.total_minted
        .checked_add(1)
        .ok_or(GameError::MathOverflow)?;

    msg!("Soul Stone minted successfully to player");

    Ok(())
}