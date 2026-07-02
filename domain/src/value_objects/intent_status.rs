
//somente status de intent (geral)
pub enum IntentStatus {
    Pending,        // MP: action_required / PM: waiting_payment
    Paid,           // MP: approved      / PMpaid: 
    Failed,         // MP: rejected      / PM: failed | with_error
    Refunded,       // MP: refunded      / PM: refunded
    PendingRefund,  // PM: pending_refund (MP não tem equivalente direto)
    InvalidResponse // Se a resposta não é mapeada
}
