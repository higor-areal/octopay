use crate::entities::{error::DomainError, events::GatewayEvent, models::gateway_attempt::GatewayAttemptEventResult};


pub trait GatewayAttemptEventRouter {
    fn route_event(
        &mut self,
        event: GatewayEvent,
    ) -> Result<GatewayAttemptEventResult, DomainError>;
}

pub trait GatewayEventHandler {
    type Event;

    fn apply_event(
        &mut self,
        event: Self::Event,
    ) -> GatewayAttemptEventResult;
}