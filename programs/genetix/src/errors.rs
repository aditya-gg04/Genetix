use anchor_lang::prelude::*;

#[error_code]
pub enum GameError {
    #[msg("Unauthorized: Only admin can perform this action")]
    Unauthorized,
    
    #[msg("Invalid fee percentage: Must be between 0 and 10000 basis points")]
    InvalidFeePercentage,
    
    #[msg("Template already exists")]
    TemplateAlreadyExists,
    
    #[msg("Template not found or inactive")]
    TemplateNotFound,
    
    #[msg("Insufficient MON tokens")]
    InsufficientMonTokens,
    
    #[msg("Insufficient SOL for Soul Stone purchase")]
    InsufficientSol,
    
    #[msg("No Soul Stone available to burn")]
    NoSoulStone,
    
    #[msg("Battle is already full")]
    BattleAlreadyFull,
    
    #[msg("Battle not ready to resolve")]
    BattleNotReady,
    
    #[msg("Battle already resolved")]
    BattleAlreadyResolved,
    
    #[msg("Invalid battle status")]
    InvalidBattleStatus,
    
    #[msg("You are not a participant in this battle")]
    NotBattleParticipant,
    
    #[msg("Cannot join your own battle")]
    CannotJoinOwnBattle,
    
    #[msg("Invalid Pokémon mint")]
    InvalidPokemonMint,
    
    #[msg("Pokémon does not belong to you")]
    NotPokemonOwner,
    
    #[msg("Mathematical overflow occurred")]
    MathOverflow,
    
    #[msg("Invalid metadata URI")]
    InvalidMetadataUri,
    
    #[msg("Name too long")]
    NameTooLong,
    
    #[msg("URI too long")]
    UriTooLong,
    
    #[msg("Invalid stats: Stats must be greater than 0")]
    InvalidStats,
    
    #[msg("Price must be greater than 0")]
    InvalidPrice,
    
    #[msg("Insufficient balance for withdrawal")]
    InsufficientBalance,

    #[msg("Invalid metadata account")]
    InvalidMetadataAccount,
}