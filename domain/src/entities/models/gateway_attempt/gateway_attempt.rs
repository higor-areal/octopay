use std::todo;

use crate::{entities::{contracts::{gateway_attempt_event::{GatewayAttemptEventRouter, GatewayEventHandler}, gateway_attempt_rules::{GatewayAttemptActions, GatewayAttemptState}}, error::DomainError, events::gateway_attempt::GatewayEvent, models::gateway_attempt::{PixGatewayAttempt, boleto::BoletoGatewayAttempt, card::CardGatewayAttempt, conflict::GatewayAttemptEventResult}}, value_objects::ids::gateway_attempt_id::GatewayAttemptId};


pub enum GatewayAttempt {
    Pix(PixGatewayAttempt),
    Card(CardGatewayAttempt),
    Boleto(BoletoGatewayAttempt),
}

impl GatewayAttempt{
    pub fn id(&self) -> GatewayAttemptId {
        match self {
            GatewayAttempt::Pix(attempt) => attempt.id(),
            GatewayAttempt::Card(attempt) => attempt.id(),
            GatewayAttempt::Boleto(attempt) => attempt.id(),
        }
    }
}

impl GatewayAttemptActions for GatewayAttempt{
    fn can_be_cancelled(&self) -> bool {
        match self {
            Self::Pix(x) => x.can_be_cancelled(),
            Self::Card(x) => x.can_be_cancelled(),
            Self::Boleto(x) => x.can_be_cancelled()
        }
    }

    fn blocks_payment_attempt(&self) -> bool {
        match self {
            Self::Pix(x) => x.blocks_payment_attempt(),
            Self::Card(x) => x.blocks_payment_attempt(),
            Self::Boleto(x) => x.blocks_payment_attempt()
        }
    }

    fn request_cancellation(&mut self) -> bool {
        match self {
            GatewayAttempt::Pix(attempt) => {
                attempt.request_cancellation()
            }

            GatewayAttempt::Card(attempt) => {
                attempt.request_cancellation()
            }

            GatewayAttempt::Boleto(attempt) => {
                attempt.request_cancellation()
            }
        }
    }
}

impl GatewayAttemptState for GatewayAttempt{
    fn is_paid(&self) -> bool {
        todo!()
    }

    fn is_failed(&self) -> bool {
        todo!()
    }

    fn is_cancelled(&self) -> bool {
        todo!("is_cancelled")
    }

    fn is_finished(&self) -> bool {
        match self {
            Self::Pix(x) => x.is_finished(),
            Self::Card(x) => x.is_finished(),
            Self::Boleto(x) => x.is_finished()
        }
    }
}

impl GatewayAttemptEventRouter for GatewayAttempt {
    fn route_event(
        &mut self,
        event: GatewayEvent,
    ) -> Result<GatewayAttemptEventResult, DomainError> {
        match (self, event) {
            (
                GatewayAttempt::Pix(attempt),
                GatewayEvent::Pix(event),
            ) => {
                Ok(attempt.apply_event(event))
            }

            (
                GatewayAttempt::Card(attempt),
                GatewayEvent::Card(event),
            ) => {
                Ok(attempt.apply_event(event))
            }

            (
                GatewayAttempt::Boleto(attempt),
                GatewayEvent::Boleto(event),
            ) => {
                Ok(attempt.apply_event(event))
            }

            _ => Err(DomainError::InvalidGatewayEvent),
        }
    }
}