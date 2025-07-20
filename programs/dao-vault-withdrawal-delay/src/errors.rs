use anchor_lang::prelude::*;

#[error_code]
pub enum MultisigError {
    #[msg("Invalid threshold")]
    InvalidThreshold,
    #[msg("Invalid signer")]
    InvalidSigner,
    #[msg("Insufficient signatures")]
    InsufficientSignatures,
    #[msg("Multisig not initialized")]
    NotInitialized,
    #[msg("Signer already approved")]
    AlreadyApproved,
    #[msg("Too many signers for bitfield")]
    TooManySigners,
    #[msg("Multisig already initialized")]
    AlreadyInitialized,
    #[msg("Early execute")]
    EarlyExecute,
}