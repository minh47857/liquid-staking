use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("User have already unstaked before")]
    UserAlreadyUnstaked,

    #[msg("Unbound delay not passed")]
    UnboundDelayNotPassed,
}
