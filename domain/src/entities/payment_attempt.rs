use crate::{
    entities::gateway_attempt::{GatewayAttempt, lifecycle::AttemptLifecycle}, value_objects::{
        ids::{
            payment_attempt_id::PaymentAttemptId, 
            payment_intent_id::PaymentIntentId
    }, 
    payment_method::instruction::payment_method::PaymentMethod, 
    intent_status::IntentStatus
}};


#[allow(dead_code)]
pub struct PaymentAttempt {
    id: PaymentAttemptId,
    payment_intent_id: PaymentIntentId,
    payment_method: PaymentMethod,
    status: IntentStatus,
    gateway_attempts: Vec<GatewayAttempt>,
}

impl PaymentAttempt {
    fn recalculate_status<T>(attempts: &[T]) -> IntentStatus
    where
        T: AttemptLifecycle,
    {
        let mut has_running = false;

        for attempt in attempts {
            if attempt.is_success() { return IntentStatus::Paid; }
            if attempt.is_running() { has_running = true; }
        }

        if !has_running { return IntentStatus::Failed; }

        IntentStatus::Pending
    }

    pub fn charge_status(&mut self) -> &IntentStatus{
        let status_now = Self::recalculate_status(&self.gateway_attempts);
        self.status = status_now;
        &self.status
    }
}
