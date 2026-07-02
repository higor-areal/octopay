use crate::{entities::gateway_attempt::lifecycle::AttemptLifecycle, gateway::gateway_name::GatewayName, value_objects::{ids::{gateway_attempt_id::GatewayAttemptId, payment_attempt_id::PaymentAttemptId}, payment_method::outcome::pix::PixData}};

pub struct PixGatewayAttempt {
    pub id: GatewayAttemptId,
    pub payment_attempt_id: PaymentAttemptId,
    pub gateway_name: GatewayName,
    pub outcome: PixData,        // outcome/ do value_object
    pub status: PixAttemptStatus,
}

pub enum PixAttemptStatus {
    Pending,
    Processing,
    RacingWinner,   // chegou primeiro — confirma
    RacingLoser,    // chegou segundo — solicitar cancelamento
    Failed,
    Cancelled,
}

impl PixGatewayAttempt {

    fn is_running(&self) -> bool {
        matches!(self.status,
            PixAttemptStatus::Processing
            | PixAttemptStatus::Pending

        )
    }

    fn is_success(&self) -> bool {
        matches!(self.status,
            PixAttemptStatus::RacingWinner
        )
    }

    fn is_failure(&self) -> bool {
        matches!(self.status,
            | PixAttemptStatus::Failed
            | PixAttemptStatus::Cancelled
        )
    }
}

impl AttemptLifecycle for PixGatewayAttempt{
    fn is_running(&self) -> bool {
        self.is_running()
    }
    fn is_success(&self) -> bool {
        self.is_success()
    }
    fn is_failure(&self) -> bool {
        self.is_failure()
    }
}