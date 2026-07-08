use crate::{entities::{models::{payment_attempt::PaymentAttempt, status::PaymentIntentStatus}}, value_objects::{amount::Amount, ids::payment_intent_id::PaymentIntentId, payer::payer::Payer}};


#[allow(dead_code)]
pub struct PaymentIntent {
    id: PaymentIntentId,
    amount: Amount,
    payer: Payer,
    status: PaymentIntentStatus,
    attempts: Vec<PaymentAttempt>
}

impl PaymentIntent{
    pub fn new (
        id: PaymentIntentId,
        amount: Amount,
        payer: Payer,
        status: PaymentIntentStatus,
        attempts: Vec<PaymentAttempt>
    )-> Self{
        Self {
            id,
            amount,
            payer,
            status,
            attempts
        }
    }

    pub fn status(&self) -> PaymentIntentStatus{
        self.status
    }

    
}
