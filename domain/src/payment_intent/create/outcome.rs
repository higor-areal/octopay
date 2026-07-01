use crate::value_objects::{
    ids::payment_intent_id::PaymentIntentId, 
    payment_method::outcome::payment_method_outcome::PaymentMethodOutcome, 
    payment_status::PaymentStatus
};


#[allow(dead_code)]
pub struct Outcome{
    id: PaymentIntentId,
    status: PaymentStatus,
    payment_method: PaymentMethodOutcome
}