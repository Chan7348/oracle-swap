use anchor_lang::prelude::*;
#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient payment provided.")]
    InsufficientPayment,
    #[msg("Transfer failed.")]
    TransferFailed,
}