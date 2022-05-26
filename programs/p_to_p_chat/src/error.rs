use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Name is more than 128 bytes")]
    NameTooLong,
    #[msg("Location is more than 32 bytes")]
    LocationTooLong,
    #[msg("Message is more than 512 bytes")]
    MessageTooLong,
}