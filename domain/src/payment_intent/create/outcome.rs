use crate::{entities::models::status::PaymentIntentStatus, value_objects::{
    ids::payment_intent_id::PaymentIntentId, 
    payment_method::outcome::payment_method_outcome::PaymentMethodOutcome, 
}};


#[allow(dead_code)]
pub struct Outcome{
    id: PaymentIntentId,
    status: PaymentIntentStatus,
    payment_method: PaymentMethodOutcome
}