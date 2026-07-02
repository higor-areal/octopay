use crate::entities::gateway_attempt::{boleto::BoletoGatewayAttempt, card::CardGatewayAttempt, lifecycle::AttemptLifecycle, pix::PixGatewayAttempt};

pub mod pix;
pub mod card;
pub mod boleto;
pub mod lifecycle;

pub enum GatewayAttempt {
    Pix(PixGatewayAttempt),
    Card(CardGatewayAttempt),
    Boleto(BoletoGatewayAttempt),
}

impl AttemptLifecycle for GatewayAttempt{
    fn is_running(&self) -> bool {
        match self {
            GatewayAttempt::Pix(a) => a.is_running(),
            GatewayAttempt::Card(a) => a.is_running(),
            GatewayAttempt::Boleto(a) => a.is_running(),
        }
    }

    fn is_success(&self) -> bool {
        match self {
            GatewayAttempt::Pix(a) => a.is_success(),
            GatewayAttempt::Card(a) => a.is_success(),
            GatewayAttempt::Boleto(a) => a.is_success(),
        }
    }

    fn is_failure(&self) -> bool {
        match self {
            GatewayAttempt::Pix(a) => a.is_failure(),
            GatewayAttempt::Card(a) => a.is_failure(),
            GatewayAttempt::Boleto(a) => a.is_failure(),
        }
    }
}