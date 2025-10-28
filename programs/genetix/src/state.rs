use anchor_lang::prelude::*;

/// Platform configuration and settings
#[account]
#[derive(InitSpace)]
pub struct Platform {
    pub admin: Pubkey,
    pub mon_token_mint: Pubkey,
    pub soul_stone_mint: Pubkey,
    pub soul_stone_price_lamports: u64,
    pub platform_fee_percentage: u16, // Basis points (e.g., 500 = 5%)
    pub total_pokemon_minted: u64,
    pub total_battles: u64,
    pub bump: u8,
}

/// Pokémon template in the marketplace
#[account]
#[derive(InitSpace)]
pub struct PokemonTemplate {
    pub template_id: u64,
    #[max_len(32)]
    pub name: String,
    #[max_len(200)]
    pub base_uri: String,
    pub price_in_mon: u64,
    pub hp: u16,
    pub attack: u16,
    pub defense: u16,
    pub speed: u16,
    pub is_active: bool,
    pub times_minted: u64,
    pub bump: u8,
}

/// Individual Pokémon NFT data
#[account]
#[derive(InitSpace)]
pub struct PokemonData {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub template_id: u64,
    #[max_len(32)]
    pub name: String,
    #[max_len(200)]
    pub current_metadata_uri: String,
    pub hp: u16,
    pub attack: u16,
    pub defense: u16,
    pub speed: u16,
    pub level: u8,
    pub evolution_stage: u8,
    pub battles_won: u32,
    pub battles_lost: u32,
    pub created_at: i64,
    pub last_battle_at: i64,
    pub bump: u8,
}

/// PvP Battle escrow
#[account]
#[derive(InitSpace)]
pub struct Battle {
    pub battle_id: u64,
    pub player1: Pubkey,
    pub player2: Option<Pubkey>,
    pub player1_pokemon: Pubkey,
    pub player2_pokemon: Option<Pubkey>,
    pub stake_amount: u64, // 10 MON tokens per player
    pub platform_fee_amount: u64,
    pub status: BattleStatus,
    pub winner: Option<Pubkey>,
    pub created_at: i64,
    pub resolved_at: Option<i64>,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum BattleStatus {
    WaitingForPlayer2,
    InProgress,
    Resolved,
    Cancelled,
}

/// Soul Stone minting configuration
#[account]
#[derive(InitSpace)]
pub struct SoulStoneConfig {
    pub mint: Pubkey,
    pub price_in_lamports: u64,
    pub total_minted: u64,
    pub bump: u8,
}

/// Platform treasury for fee collection
#[account]
#[derive(InitSpace)]
pub struct PlatformTreasury {
    pub total_fees_collected: u64,
    pub mon_token_vault: Pubkey,
    pub bump: u8,
}