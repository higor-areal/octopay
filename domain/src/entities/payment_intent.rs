use crate::{entities::payment_attempt::PaymentAttempt, value_objects::{amount::Amount, ids::payment_intent_id::PaymentIntentId, payer::payer::Payer, intent_status::IntentStatus}};


#[allow(dead_code)]
pub struct PaymentIntent {
    id: PaymentIntentId,
    amount: Amount,
    payer: Payer,
    status: IntentStatus,
    attempts: Vec<PaymentAttempt>
}