use crate::{ entities::models::gateway_attempt::GatewayAttempt, value_objects::ids::gateway_attempt_id::GatewayAttemptId};
                                                                  
pub enum PaymentAttemptEvent {
    CancellationRequested(GatewayAttemptId),
    AttemptRequested(GatewayAttempt),
}