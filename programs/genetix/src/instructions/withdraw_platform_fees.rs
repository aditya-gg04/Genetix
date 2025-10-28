use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};
use crate::{state::*, errors::GameError};

#[derive(Accounts)]
pub struct WithdrawPlatformFees<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        seeds = [b"platform"],
        bump = platform.bump,
        has_one = admin @ GameError::Unauthorized
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
        address = platform.mon_token_mint
    )]
    pub mon_token_mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mon_token_mint,
        associated_token::authority = platform
    )]
    pub platform_fee_account: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = admin,
        associated_token::mint = mon_token_mint,
        associated_token::authority = admin
    )]
    pub admin_mon_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn withdraw_platform_fees(
    ctx: Context<WithdrawPlatformFees>,
    amount: u64,
) -> Result<()> {
    require!(amount > 0, GameError::InvalidPrice);
    require!(
        ctx.accounts.platform_fee_account.amount >= amount,
        GameError::InsufficientBalance
    );

    let platform_seeds = &[
        b"platform".as_ref(),
        &[ctx.accounts.platform.bump],
    ];
    let signer_seeds = &[&platform_seeds[..]];

    // Transfer fees to admin
    transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.platform_fee_account.to_account_info(),
                to: ctx.accounts.admin_mon_account.to_account_info(),
                authority: ctx.accounts.platform.to_account_info(),
            },
            signer_seeds,
        ),
        amount,
    )?;

    msg!("Withdrew {} MON tokens from platform fees", amount);

    Ok(())
}