use crate::value_objects::{
    ids::payment_intent_id::PaymentIntentId, 
    payment_method::outcome::payment_method_outcome::PaymentMethodOutcome, 
    intent_status::IntentStatus
};


#[allow(dead_code)]
pub struct Outcome{
    id: PaymentIntentId,
    status: IntentStatus,
    payment_method: PaymentMethodOutcome
}