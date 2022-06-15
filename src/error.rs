use serde_json::error::Error as SerdeError;
use std::result;
use std::time::SystemTimeError;
use thiserror::Error;
use tungstenite::error::Error as TungsteniteError;

pub type Result<T, E = BybitError> = result::Result<T, E>;

#[derive(Error, Debug)]
pub enum BybitError {
    #[error("Serde error: {0}")]
    SerdeError(#[from] SerdeError),

    #[error("Tungstenite error: {0}")]
    TungsteniteError(#[from] TungsteniteError),

    #[error("System time error: {0}")]
    SystemTimeError(#[from] SystemTimeError),

    #[error("WebSocket URI parse error: {0}")]
    BadWebSocketURI(#[from] url::ParseError),
}
