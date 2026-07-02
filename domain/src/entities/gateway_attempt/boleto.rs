use crate::{
    entities::gateway_attempt::lifecycle::AttemptLifecycle, gateway::gateway_name::GatewayName, value_objects::{
        ids::{
            gateway_attempt_id::GatewayAttemptId, 
        payment_attempt_id::PaymentAttemptId
    }, 
        payment_method::outcome::boleto::BoletoData
    }
};

pub struct BoletoGatewayAttempt {
    pub id: GatewayAttemptId,
    pub payment_attempt_id: PaymentAttemptId,
    pub gateway_name: GatewayName,
    pub outcome: BoletoData,
    pub status: BoletoAttemptStatus,
}

pub enum BoletoAttemptStatus {
    //running
    Pending,

    //sucess
    Paid,

    //failure
    Expired,
    Cancelled,
}

impl BoletoGatewayAttempt{
    fn is_running(&self) -> bool {
        matches!(self.status, 
            BoletoAttemptStatus::Pending
        )
    }
    fn is_success(&self) -> bool {
        matches!(self.status, 
            BoletoAttemptStatus::Paid
        )
    }
    fn is_failure(&self) -> bool {
        matches!(self.status, 
            BoletoAttemptStatus::Expired
            | BoletoAttemptStatus::Cancelled
        )
    }
}

impl AttemptLifecycle for BoletoGatewayAttempt{
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