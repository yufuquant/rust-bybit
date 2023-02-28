use serde_json::error::Error as SerdeError;
use std::result;
use thiserror::Error;
use tungstenite::error::Error as TungsteniteError;

pub type Result<T, E = BybitError> = result::Result<T, E>;

#[derive(Error, Debug)]
pub enum BybitError {
    #[error("Serde error: {0}")]
    SerdeError(#[from] SerdeError),

    #[error("Tungstenite error: {0}")]
    TungsteniteError(#[from] TungsteniteError),
}
