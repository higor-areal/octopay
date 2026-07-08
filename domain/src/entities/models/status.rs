
#[derive(PartialEq, Eq)]
pub enum PaymentAttemptStatus {
    Pending,
    Paid, 
    Failed,
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum PaymentIntentStatus {
    Pending,
    Paid,
    Failed,
    Cancelled,
}