use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("TokenSynchronize: Invalid token program")]
    InvalidTokenProgram,
    
    #[msg("TokenSynchronize: Amount must be greater than zero")]
    ZeroAmount,

    #[msg("TokenSynchronize: Insufficient funds")]
    InsufficientFunds,

    #[msg("TokenSynchronize: Cannot approve SOL transfers")]
    CannotApproveSol,

    #[msg("TokenSynchronize: No delegate set")]
    NoDelegate,

    #[msg("TokenSynchronize: Delegate and authority must be different")]
    DelegateEqualsAuthority,

    #[msg("TokenSynchronize: Delegate and authority must be the same")]
    DelegateNotAuthority,
}