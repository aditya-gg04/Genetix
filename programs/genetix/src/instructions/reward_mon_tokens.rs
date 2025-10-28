use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};
use crate::{state::Platform, errors::GameError};

#[derive(Accounts)]
pub struct RewardMonTokens<'info> {
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
        address = platform.mon_token_mint
    )]
    pub mon_token_mint: Account<'info, Mint>,

    /// CHECK: Recipient player
    pub recipient: UncheckedAccount<'info>,

    #[account(
        init_if_needed,
        payer = admin,
        associated_token::mint = mon_token_mint,
        associated_token::authority = recipient
    )]
    pub recipient_mon_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn reward_mon_tokens(
    ctx: Context<RewardMonTokens>,
    amount: u64,
) -> Result<()> {
    require!(amount > 0, GameError::InvalidPrice);

    let platform_seeds = &[
        b"platform".as_ref(),
        &[ctx.accounts.platform.bump],
    ];
    let signer_seeds = &[&platform_seeds[..]];

    // Mint MON tokens as reward
    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mon_token_mint.to_account_info(),
                to: ctx.accounts.recipient_mon_account.to_account_info(),
                authority: ctx.accounts.platform.to_account_info(),
            },
            signer_seeds,
        ),
        amount,
    )?;

    msg!("Rewarded {} MON tokens to {}", amount, ctx.accounts.recipient.key());

    Ok(())
}