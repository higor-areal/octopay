use crate::{
    entities::events::gateway_attempt::GatewayEvent,
    value_objects::ids::gateway_attempt_id::GatewayAttemptId,
};

pub enum GatewayAttemptEventResult {
    Applied,
    Conflict(GatewayAttemptConflict),
}

pub struct GatewayAttemptConflict {
    gateway_attempt_id: GatewayAttemptId,
    event: GatewayEvent,
}

impl GatewayAttemptConflict {
    pub fn new(
        gateway_attempt_id: GatewayAttemptId,
        event: GatewayEvent,
    ) -> Self {
        Self {
            gateway_attempt_id,
            event,
        }
    }

    pub fn into_parts(
        self,
    ) -> (GatewayAttemptId, GatewayEvent) {
        (
            self.gateway_attempt_id,
            self.event,
        )
    }
}