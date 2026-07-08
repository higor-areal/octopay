use thiserror::Error;

use crate::{entities::error::DomainError, value_objects::error::ValidationError};


#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    Validation(#[from] ValidationError),

    #[error(transparent)]
    Domain(#[from] DomainError),
}