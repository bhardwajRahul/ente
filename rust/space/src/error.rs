use ente_core::{crypto, http};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SpaceError {
    #[error(transparent)]
    Http(#[from] http::Error),

    #[error(transparent)]
    Crypto(#[from] crypto::Error),

    #[error("invalid input: {0}")]
    InvalidInput(String),

    #[error("missing secret key material")]
    MissingSecretKey,

    #[error("missing friend sealed space key")]
    MissingFriendSealedSpaceKey,

    #[error("entity key conflict")]
    EntityKeyConflict,
}

impl SpaceError {
    pub fn is_unavailable_record(&self) -> bool {
        matches!(
            self,
            Self::Crypto(_) | Self::InvalidInput(_) | Self::MissingFriendSealedSpaceKey
        )
    }
}

pub type Result<T> = std::result::Result<T, SpaceError>;
