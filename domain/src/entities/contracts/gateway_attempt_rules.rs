pub trait GatewayAttemptState {
    fn is_paid(&self) -> bool;
    fn is_failed(&self) -> bool;
    fn is_cancelled(&self) -> bool;
    fn is_finished(&self) -> bool;
}

pub trait GatewayAttemptActions {
    fn can_be_cancelled(&self) -> bool;
    fn blocks_payment_attempt(&self) -> bool;
    fn request_cancellation(&mut self) -> bool;
}

