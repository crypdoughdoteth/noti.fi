use sui_sdk::error::Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Errors {
    #[error(transparent)]
    SuiError(#[from] Error),
    #[error("Failed to parse str into SuiFeed")]
    SuiFeedIdParsingError,
}
