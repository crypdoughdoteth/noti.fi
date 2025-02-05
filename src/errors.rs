use thiserror::Error;
use sui_sdk::error::Error;

#[derive(Debug, Error)]
pub enum Errors {
    #[error(transparent)]
    SuiError(#[from] Error),
    #[error("Failed to parse str into SuiFeed")]
    SuiFeedIdParsingError
}
