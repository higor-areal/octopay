use crate::entities::events::payment_attempt::PaymentAttemptEvent;

pub enum PaymentIntentEvent {
    CancellationRequested,
    PaymentAttemptEvent(PaymentAttemptEvent)
}