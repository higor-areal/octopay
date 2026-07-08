use crate::{
    entities::{contracts::gateway_attempt_rules::{GatewayAttemptActions, GatewayAttemptState}, error::DomainError, events::payment_attempt::PaymentAttemptEvent, models::{ gateway_attempt::gateway_attempt::GatewayAttempt, status::PaymentAttemptStatus}}, value_objects::{
         ids::{
            gateway_attempt_id::GatewayAttemptId, payment_attempt_id::PaymentAttemptId, payment_intent_id::PaymentIntentId,
        }, payment_method::instruction::payment_method::PaymentMethod,
    },
};

#[allow(dead_code)]
pub struct PaymentAttempt {
    id: PaymentAttemptId,
    payment_intent_id: PaymentIntentId,
    payment_method: PaymentMethod,
    status: PaymentAttemptStatus,
    gateway_attempts: Vec<GatewayAttempt>,
}

impl PaymentAttempt {
    pub fn new(
        id: PaymentAttemptId,
        payment_intent_id: PaymentIntentId,
        payment_method: PaymentMethod,
        status: PaymentAttemptStatus,
        gateway_attempts: Vec<GatewayAttempt>,
    ) -> Self {
        Self {
            id,
            payment_intent_id,
            payment_method,
            status,
            gateway_attempts,
        }
    }

    pub fn status(&self) -> &PaymentAttemptStatus {
        &self.status
    }

    fn sync_status(&mut self) {
        self.status = self.calculate_status();
    }

    fn calculate_status(&self) -> PaymentAttemptStatus {
        if self.is_paid() {
            return PaymentAttemptStatus::Paid;
        }

        if self.has_running_gateway_attempt() {
            return PaymentAttemptStatus::Pending;
        }

        PaymentAttemptStatus::Failed
    }

    fn has_running_gateway_attempt(&self) -> bool {
        self.gateway_attempts
            .iter()
            .any(|gateway_attempt| !gateway_attempt.is_finished())
    }
}

impl PaymentAttempt {
    pub fn can_create_new_attempt(&self) -> bool {
        if self.is_paid() {
            return false;
        }

        !self
            .gateway_attempts
            .iter()
            .any(|gateway_attempt| gateway_attempt.blocks_payment_attempt())
    }

    pub fn can_be_cancelled(&self) -> bool {
        if self.is_paid() {
            return false;
        }

        self.gateway_attempts
            .iter()
            .any(|gateway_attempt| gateway_attempt.can_be_cancelled())
    }

    pub fn is_finished(&self) -> bool {
        self.gateway_attempts
            .iter()
            .all(|gateway_attempt| gateway_attempt.is_finished())
    }

    pub fn is_paid(&self) -> bool {
        self.gateway_attempts
            .iter()
            .any(|gateway_attempt| gateway_attempt.is_paid())
    }

    pub fn is_failed(&self) -> bool {
        self.is_finished() && !self.is_paid()
    }
}

impl PaymentAttempt {
    fn add_gateway_attempt(
        &mut self,
        attempt: GatewayAttempt,
    ) -> Result<(), DomainError> {
        if !self.can_create_new_attempt() {
            return Err(DomainError::PaymentAlreadyCompleted);
        }

        self.gateway_attempts.push(attempt);
        self.sync_status();

        Ok(())
    }

    pub fn apply_event(
        &mut self,
        event: PaymentAttemptEvent,
    ) -> Result<(), DomainError> {
        match event {
            PaymentAttemptEvent::CancellationRequested(gateway_attempt_id) => {
                self.request_cancellation(gateway_attempt_id)
            }

            PaymentAttemptEvent::AttemptRequested(attempt) => {
                self.add_gateway_attempt(attempt)
            }
        }
    }
    
    fn request_cancellation(
        &mut self,
        gateway_attempt_id: GatewayAttemptId,
    ) -> Result<(), DomainError> {
        let gateway_attempt = self
            .gateway_attempts
            .iter_mut()
            .find(|attempt| attempt.id() == gateway_attempt_id)
            .ok_or(DomainError::GatewayAttemptNotFound)?;

        if !gateway_attempt.can_be_cancelled() {
            return Err(DomainError::GatewayAttemptCannotBeCancelled.into());
        }

        gateway_attempt.request_cancellation();

        self.sync_status();

        Ok(())
    }
}
