use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Metadata URI is too long")]
    MetadataTooLong,
}
