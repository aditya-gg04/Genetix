use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod errors;

pub use instructions::*;
pub use state::*;
pub use errors::*;

declare_id!("5a9hz3a3PnFfLzc5tzPChZKCSnJTdf1oTLmXzTGaRryv");

pub const ANCHOR_DISCRIMINATOR: usize = 8;
pub const MON_TOKEN_DECIMALS: u8 = 9;
pub const SOUL_STONE_DECIMALS: u8 = 0;

#[program]
pub mod pokemon_game {
    use super::*;

    // ============ ADMIN INSTRUCTIONS ============
    
    /// Initialize the game platform with admin and fee settings
    pub fn initialize_platform(
        ctx: Context<InitializePlatform>,
        platform_fee_percentage: u16,
    ) -> Result<()> {
        instructions::initialize_platform::initialize_platform(ctx, platform_fee_percentage)
    }

    /// Create the MON token (ERC20-like utility token)
    pub fn create_mon_token(
        ctx: Context<CreateMonToken>,
        token_name: String,
        token_symbol: String,
        token_uri: String,
    ) -> Result<()> {
        instructions::create_mon_token::create_mon_token(ctx, token_name, token_symbol, token_uri)
    }

    /// Create the Soul Stone token
    pub fn create_soul_stone_token(
        ctx: Context<CreateSoulStoneToken>,
        token_name: String,
        token_symbol: String,
        token_uri: String,
    ) -> Result<()> {
        instructions::create_soul_stone_token::create_soul_stone_token(ctx, token_name, token_symbol, token_uri)
    }

    /// Set the price for minting Soul Stones in SOL
    pub fn set_soul_stone_price(
        ctx: Context<SetSoulStonePrice>,
        price_in_lamports: u64,
    ) -> Result<()> {
        instructions::set_soul_stone_price::set_soul_stone_price(ctx, price_in_lamports)
    }

    /// Add a Pokémon template to the marketplace
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
        instructions::add_pokemon_template::add_pokemon_template(
            ctx,
            template_id,
            name,
            base_uri,
            price_in_mon,
            hp,
            attack,
            defense,
            speed,
        )
    }

    /// Update platform fee percentage
    pub fn update_platform_fee(
        ctx: Context<UpdatePlatformFee>,
        new_fee_percentage: u16,
    ) -> Result<()> {
        instructions::update_platform_fee::update_platform_fee(ctx, new_fee_percentage)
    }

    // ============ USER INSTRUCTIONS ============

    /// Mint a new Pokémon NFT from a template
    pub fn mint_pokemon(
        ctx: Context<MintPokemon>,
        template_id: u64,
    ) -> Result<()> {
        instructions::mint_pokemon::mint_pokemon(ctx, template_id)
    }

    /// Mint a Soul Stone by paying SOL
    pub fn mint_soul_stone(ctx: Context<MintSoulStone>) -> Result<()> {
        instructions::mint_soul_stone::mint_soul_stone(ctx)
    }

    /// Evolve a Pokémon by burning a Soul Stone
    pub fn evolve_pokemon(
        ctx: Context<EvolvePokemon>,
        new_metadata_uri: String,
        new_hp: u16,
        new_attack: u16,
        new_defense: u16,
        new_speed: u16,
    ) -> Result<()> {
        instructions::evolve_pokemon::evolve_pokemon(
            ctx,
            new_metadata_uri,
            new_hp,
            new_attack,
            new_defense,
            new_speed,
        )
    }

    /// Create a PvP battle by staking MON tokens
    pub fn create_battle(
        ctx: Context<CreateBattle>,
        battle_id: u64,
        pokemon_mint: Pubkey,
    ) -> Result<()> {
        instructions::create_battle::create_battle(ctx, battle_id, pokemon_mint)
    }

    /// Join an existing battle by staking MON tokens
    pub fn join_battle(
        ctx: Context<JoinBattle>,
        pokemon_mint: Pubkey,
    ) -> Result<()> {
        instructions::join_battle::join_battle(ctx, pokemon_mint)
    }

    /// Resolve a battle and distribute rewards
    pub fn resolve_battle(
        ctx: Context<ResolveBattle>,
        winner_is_player1: bool,
    ) -> Result<()> {
        instructions::resolve_battle::resolve_battle(ctx, winner_is_player1)
    }

    /// Reward MON tokens to a player (for winning battles/defeating bosses)
    pub fn reward_mon_tokens(
        ctx: Context<RewardMonTokens>,
        amount: u64,
    ) -> Result<()> {
        instructions::reward_mon_tokens::reward_mon_tokens(ctx, amount)
    }

    /// Update Pokémon metadata URI
    pub fn update_pokemon_metadata(
        ctx: Context<UpdatePokemonMetadata>,
        new_uri: String,
    ) -> Result<()> {
        instructions::update_pokemon_metadata::update_pokemon_metadata(ctx, new_uri)
    }

    /// Withdraw platform fees (admin only)
    pub fn withdraw_platform_fees(
        ctx: Context<WithdrawPlatformFees>,
        amount: u64,
    ) -> Result<()> {
        instructions::withdraw_platform_fees::withdraw_platform_fees(ctx, amount)
    }
}