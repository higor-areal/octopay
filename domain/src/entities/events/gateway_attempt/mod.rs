
pub mod pix;
pub mod card;
pub mod boleto;

pub use pix::PixGatewayEvent;
pub use card::CardGatewayEvent;
pub use boleto::BoletoGatewayEvent;


pub enum GatewayEvent{
    Pix(PixGatewayEvent),
    Card(CardGatewayEvent),
    Boleto(BoletoGatewayEvent)
}

/* 
use crate::value_objects::ids::{gateway_attempt_id::GatewayAttemptId, payment_attempt_id::PaymentAttemptId};


pub struct GatewayEventMessage {
    payment_attempt_id: PaymentAttemptId,
    gateway_attempt_id: GatewayAttemptId,
    event: GatewayEvent,
}
*/