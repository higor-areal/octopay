use crate::gateway::gateway_name::GatewayName;

pub enum PaymentStatus {
    Pending,        // MP: action_required / PM: waiting_payment
    Paid,           // MP: approved      / PMpaid: 
    Failed,         // MP: rejected      / PM: failed | with_error
    Refunded,       // MP: refunded      / PM: refunded
    PendingRefund,  // PM: pending_refund (MP não tem equivalente direto)
    Expired,        // Pix vencido sem pagamento
    InvalidResponse // Se a resposta não é mapeada
}

#[allow(dead_code)]
impl PaymentStatus{
    pub fn from_gateway(gateway: GatewayName, status: &str) -> Self{
        match gateway {
            GatewayName::MercadoPago =>Self::from_mercado_pago(status),
            GatewayName::Pagarme => Self::from_pagarme(status),
            GatewayName::Stripe => Self::from_stripe(status)
        }
    }
}

#[allow(dead_code)]
impl PaymentStatus {
    fn from_mercado_pago(status: &str) -> Self {
        match status {
            "approved" => Self::Paid,

            "action_required" => Self::Pending,
            "pending" => Self::Pending,
            "in_process" => Self::Pending,

            "rejected" => Self::Failed,
            "cancelled" => Self::Failed,
            "charged_back" => Self::Failed,

            "refunded" => Self::Refunded,

            "expired" => Self::Expired,

            _ => Self::InvalidResponse,
        }
    }

    fn from_pagarme(status: &str) -> Self {
        todo!("implement pagarme status mapping:\n {status}");
    }

    fn from_stripe(status: &str) -> Self {
        todo!("implement pagarme status mapping:\n {status}");
    }
}