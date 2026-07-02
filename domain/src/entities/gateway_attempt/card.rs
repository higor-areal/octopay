use crate::{entities::gateway_attempt::lifecycle::AttemptLifecycle, gateway::gateway_name::GatewayName, value_objects::{ids::{gateway_attempt_id::GatewayAttemptId, payment_attempt_id::PaymentAttemptId}, payment_method::outcome::card::CardData}};

pub struct CardGatewayAttempt {
    pub id: GatewayAttemptId,
    pub payment_attempt_id: PaymentAttemptId,
    pub gateway_name: GatewayName,
    pub outcome: CardData,
    pub status: CardAttemptStatus,
//    pub failure_reason: Option<FailureReason>, // só preenchido em Failed
}

pub enum CardAttemptStatus {
    //is_running
    Pending,
    Processing,

    //is_sucess
    Paid,

    //is_failure
    Failed,
    Cancelled,
    Rejected,
}

impl CardGatewayAttempt {
    fn is_running(&self) -> bool{
        matches!(self.status, 
            CardAttemptStatus::Processing 
            | CardAttemptStatus::Pending
        )
    }
    fn is_success(&self) -> bool{
        matches!(self.status, CardAttemptStatus::Paid)
    }
    fn is_failure(&self) -> bool{
        matches!(self.status, 
            CardAttemptStatus::Failed
            | CardAttemptStatus::Cancelled
            | CardAttemptStatus::Rejected
        )
    }
}


impl AttemptLifecycle for CardGatewayAttempt{
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