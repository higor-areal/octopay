use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum DomainError {
    #[error("cannot create a new gateway attempt")]
    CannotCreateGatewayAttempt,

    #[error("gateway attempt cannot be cancelled")]
    GatewayAttemptCannotBeCancelled,

    #[error("gateway attempt not found")]
    GatewayAttemptNotFound,

    #[error("Payment already completed")]
    PaymentAlreadyCompleted,

     #[error("Invalid Gateway Event")]
    InvalidGatewayEvent
}