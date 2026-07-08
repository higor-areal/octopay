use std::{matches};

use crate::{ 
        entities::{contracts::{gateway_attempt_event::GatewayEventHandler, gateway_attempt_rules::{GatewayAttemptActions, GatewayAttemptState}}, events::{GatewayEvent, gateway_attempt::boleto::BoletoGatewayEvent}, models::gateway_attempt::{GatewayAttemptConflict, conflict::GatewayAttemptEventResult}}, gateway::gateway_name::GatewayName, value_objects::{
        ids::{
            gateway_attempt_id::GatewayAttemptId, 
        payment_attempt_id::PaymentAttemptId
    }, 
        payment_method::outcome::boleto::BoletoData
    }
};

#[allow(dead_code)]
pub struct BoletoGatewayAttempt {
    id: GatewayAttemptId,
    payment_attempt_id: PaymentAttemptId,
    gateway_name: GatewayName,
    outcome: BoletoData,
    status: BoletoAttemptStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoletoAttemptStatus {
    Requested, 
    Pending,
    CancellationRequested,
    Paid, //
    Cancelled, //
    Expired, //
    Failed, //
}

#[allow(dead_code)]
impl BoletoGatewayAttempt{

    pub fn new(
        id: GatewayAttemptId,
        payment_attempt_id: PaymentAttemptId,
        gateway_name: GatewayName,
        outcome: BoletoData,
    ) -> Self{
        let status = BoletoAttemptStatus::Requested;
        Self { id, 
            payment_attempt_id, 
            gateway_name, 
            outcome, 
            status 
        }
    }

    pub(crate) fn id(&self) -> GatewayAttemptId {
        self.id
    }
}

impl GatewayAttemptActions for BoletoGatewayAttempt{
    fn can_be_cancelled(&self) -> bool {
        matches!(self.status,
            BoletoAttemptStatus::Pending
        )
    }
 
    fn blocks_payment_attempt(&self) -> bool {
        matches!(self.status,
            BoletoAttemptStatus::Pending
            | BoletoAttemptStatus::Requested
            | BoletoAttemptStatus::CancellationRequested
            | BoletoAttemptStatus::Paid
        )
    }
    fn request_cancellation(&mut self) -> bool {
        if !self.can_be_cancelled() {
            return false;
        }
        self.status = BoletoAttemptStatus::CancellationRequested;
        true
    }
}

impl GatewayAttemptState for BoletoGatewayAttempt{
    fn is_paid(&self) -> bool {
        matches!(self.status,
            BoletoAttemptStatus::Paid
        )
    }

    fn is_failed(&self) -> bool {
        matches!(self.status,
            BoletoAttemptStatus::Failed
        )
    }

    fn is_cancelled(&self) -> bool {
        matches!(self.status,
            BoletoAttemptStatus::Cancelled
        )
    }

    fn is_finished(&self) -> bool {
        matches!(self.status,
            BoletoAttemptStatus::Paid
            | BoletoAttemptStatus::Failed
            | BoletoAttemptStatus::Cancelled
            | BoletoAttemptStatus::Expired
        )
    }
}


impl GatewayEventHandler for BoletoGatewayAttempt{
    type Event = BoletoGatewayEvent;

    fn apply_event(
        &mut self,
        event: Self::Event,
    ) -> GatewayAttemptEventResult {
        match self.next_state(event) {
            Ok(status) => {
                self.status = status;
                GatewayAttemptEventResult::Applied
            }
            Err(conflict) => GatewayAttemptEventResult::Conflict(conflict)
        }
    }
}

impl BoletoGatewayAttempt{
    fn next_state(
        &self,
        event: BoletoGatewayEvent,
    ) -> Result<BoletoAttemptStatus, GatewayAttemptConflict>{
        match (self.status, &event) {
            (
                BoletoAttemptStatus::Pending
                | BoletoAttemptStatus::Requested
                | BoletoAttemptStatus::CancellationRequested,
                BoletoGatewayEvent::Paid
            ) => {
                Ok(BoletoAttemptStatus::Paid)
            }

            (
                BoletoAttemptStatus::Pending
                | BoletoAttemptStatus::Requested
                | BoletoAttemptStatus::CancellationRequested,
                BoletoGatewayEvent::Cancelled
            ) => {
                Ok(BoletoAttemptStatus::Cancelled)
            }

            (
                BoletoAttemptStatus::Pending
                | BoletoAttemptStatus::Requested
                | BoletoAttemptStatus::CancellationRequested,
                BoletoGatewayEvent::Expired
            ) => {
                Ok(BoletoAttemptStatus::Expired)
            }

            (
                BoletoAttemptStatus::Pending
                | BoletoAttemptStatus::Requested
                | BoletoAttemptStatus::CancellationRequested,
                BoletoGatewayEvent::Failed
            ) => {
                Ok(BoletoAttemptStatus::Failed)
            }
            _ => {
                Err(GatewayAttemptConflict::new(
                            self.id,
                            GatewayEvent::Boleto(event),
                        )
                    )
            }
        }
    }
}

#[cfg(test)]
mod test {
    use chrono::Utc;
    use crate::value_objects::expiration::Expiration;
    use super::*;

    fn struct_valid() -> BoletoGatewayAttempt {
        let id = GatewayAttemptId::generate();
        let payment_attempt_id = PaymentAttemptId::generate();
        let gateway_name = GatewayName::MercadoPago;
        let outcome = BoletoData::new(
            "barcode",
            "digitable_line",
            None::<String>,
            Expiration::new(Utc::now()),
        );

        BoletoGatewayAttempt::new(
            id,
            payment_attempt_id,
            gateway_name,
            outcome,
        )
    }

    mod gateway_attempt_actions {
        use super::*;

        mod can_be_cancelled {
            use super::*;

            #[test]
            fn returns_true_when_status_is_pending() {
                let mut attempt = struct_valid();
                attempt.status = BoletoAttemptStatus::Pending;

                assert!(attempt.can_be_cancelled());
            }

            mod edge_cases {
                use super::*;

                #[test]
                fn returns_false_when_status_is_requested() {
                    let attempt = struct_valid();

                    assert!(!attempt.can_be_cancelled());
                }

                #[test]
                fn returns_false_when_status_is_cancellation_requested() {
                    let mut attempt = struct_valid();
                    attempt.status = BoletoAttemptStatus::CancellationRequested;

                    assert!(!attempt.can_be_cancelled());
                }

                #[test]
                fn returns_false_when_status_is_paid() {
                    let mut attempt = struct_valid();
                    attempt.status = BoletoAttemptStatus::Paid;

                    assert!(!attempt.can_be_cancelled());
                }

                #[test]
                fn returns_false_when_status_is_cancelled() {
                    let mut attempt = struct_valid();
                    attempt.status = BoletoAttemptStatus::Cancelled;

                    assert!(!attempt.can_be_cancelled());
                }

                #[test]
                fn returns_false_when_status_is_expired() {
                    let mut attempt = struct_valid();
                    attempt.status = BoletoAttemptStatus::Expired;

                    assert!(!attempt.can_be_cancelled());
                }

                #[test]
                fn returns_false_when_status_is_failed() {
                    let mut attempt = struct_valid();
                    attempt.status = BoletoAttemptStatus::Failed;

                    assert!(!attempt.can_be_cancelled());
                }
            }
        }

        mod blocks_payment_attempt {
            use super::*;

            #[test]
            fn returns_true_when_status_is_pending() {
                let mut attempt = struct_valid();
                attempt.status = BoletoAttemptStatus::Pending;

                assert!(attempt.blocks_payment_attempt());
            }

            #[test]
            fn returns_true_when_status_is_paid() {
                let mut attempt = struct_valid();
                attempt.status = BoletoAttemptStatus::Paid;

                assert!(attempt.blocks_payment_attempt());
            }

            #[test]
            fn returns_true_when_status_is_requested() {
                let attempt = struct_valid();

                assert!(attempt.blocks_payment_attempt());
            }

            #[test]
            fn returns_true_when_status_is_cancellation_requested() {
                let mut attempt = struct_valid();
                attempt.status = BoletoAttemptStatus::CancellationRequested;

                assert!(attempt.blocks_payment_attempt());
            }

            mod edge_cases {
                use super::*;

                #[test]
                fn returns_false_when_status_is_cancelled() {
                    let mut attempt = struct_valid();
                    attempt.status = BoletoAttemptStatus::Cancelled;

                    assert!(!attempt.blocks_payment_attempt());
                }

                #[test]
                fn returns_false_when_status_is_expired() {
                    let mut attempt = struct_valid();
                    attempt.status = BoletoAttemptStatus::Expired;

                    assert!(!attempt.blocks_payment_attempt());
                }

                #[test]
                fn returns_false_when_status_is_failed() {
                    let mut attempt = struct_valid();
                    attempt.status = BoletoAttemptStatus::Failed;

                    assert!(!attempt.blocks_payment_attempt());
                }
            }
        }

        mod request_cancellation {
            use super::*;

            #[test]
            fn cancels_and_returns_true_when_status_is_pending() {
                let mut attempt = struct_valid();
                attempt.status = BoletoAttemptStatus::Pending;

                let result = attempt.request_cancellation();

                assert!(result);
                assert_eq!(attempt.status, BoletoAttemptStatus::CancellationRequested);
            }

            mod edge_cases {
                use super::*;

                #[test]
                fn returns_false_and_preserves_status_when_requested() {
                    let mut attempt = struct_valid();

                    let result = attempt.request_cancellation();

                    assert!(!result);
                    assert_eq!(attempt.status, BoletoAttemptStatus::Requested);
                }

                #[test]
                fn returns_false_and_preserves_status_when_cancellation_already_requested() {
                    let mut attempt = struct_valid();
                    attempt.status = BoletoAttemptStatus::CancellationRequested;

                    let result = attempt.request_cancellation();

                    assert!(!result);
                    assert_eq!(attempt.status, BoletoAttemptStatus::CancellationRequested);
                }

                #[test]
                fn returns_false_and_preserves_status_when_paid() {
                    let mut attempt = struct_valid();
                    attempt.status = BoletoAttemptStatus::Paid;

                    let result = attempt.request_cancellation();

                    assert!(!result);
                    assert_eq!(attempt.status, BoletoAttemptStatus::Paid);
                }

                #[test]
                fn returns_false_and_preserves_status_when_cancelled() {
                    let mut attempt = struct_valid();
                    attempt.status = BoletoAttemptStatus::Cancelled;

                    let result = attempt.request_cancellation();

                    assert!(!result);
                    assert_eq!(attempt.status, BoletoAttemptStatus::Cancelled);
                }

                #[test]
                fn returns_false_and_preserves_status_when_expired() {
                    let mut attempt = struct_valid();
                    attempt.status = BoletoAttemptStatus::Expired;

                    let result = attempt.request_cancellation();

                    assert!(!result);
                    assert_eq!(attempt.status, BoletoAttemptStatus::Expired);
                }

                #[test]
                fn returns_false_and_preserves_status_when_failed() {
                    let mut attempt = struct_valid();
                    attempt.status = BoletoAttemptStatus::Failed;

                    let result = attempt.request_cancellation();

                    assert!(!result);
                    assert_eq!(attempt.status, BoletoAttemptStatus::Failed);
                }
            }
        }
    }

    mod gateway_attempt_state {
        use super::*;

        mod is_paid {
            use super::*;

            #[test]
            fn returns_true_when_status_is_paid() {
                let mut attempt = struct_valid();
                attempt.status = BoletoAttemptStatus::Paid;

                assert!(attempt.is_paid());
            }

            mod edge_cases {
                use super::*;

                #[test]
                fn returns_false_when_status_is_not_paid() {
                    let attempt = struct_valid();

                    assert!(!attempt.is_paid());
                }
            }
        }

        mod is_failed {
            use super::*;

            #[test]
            fn returns_true_when_status_is_failed() {
                let mut attempt = struct_valid();
                attempt.status = BoletoAttemptStatus::Failed;

                assert!(attempt.is_failed());
            }

            mod edge_cases {
                use super::*;

                #[test]
                fn returns_false_when_status_is_not_failed() {
                    let attempt = struct_valid();

                    assert!(!attempt.is_failed());
                }
            }
        }

        mod is_cancelled {
            use super::*;

            #[test]
            fn returns_true_when_status_is_cancelled() {
                let mut attempt = struct_valid();
                attempt.status = BoletoAttemptStatus::Cancelled;

                assert!(attempt.is_cancelled());
            }

            mod edge_cases {
                use super::*;

                #[test]
                fn returns_false_when_status_is_not_cancelled() {
                    let attempt = struct_valid();

                    assert!(!attempt.is_cancelled());
                }
            }
        }

        mod is_finished {
            use super::*;

            #[test]
            fn returns_true_when_status_is_paid() {
                let mut attempt = struct_valid();
                attempt.status = BoletoAttemptStatus::Paid;

                assert!(attempt.is_finished());
            }

            #[test]
            fn returns_true_when_status_is_failed() {
                let mut attempt = struct_valid();
                attempt.status = BoletoAttemptStatus::Failed;

                assert!(attempt.is_finished());
            }

            #[test]
            fn returns_true_when_status_is_cancelled() {
                let mut attempt = struct_valid();
                attempt.status = BoletoAttemptStatus::Cancelled;

                assert!(attempt.is_finished());
            }

            #[test]
            fn returns_true_when_status_is_expired() {
                let mut attempt = struct_valid();
                attempt.status = BoletoAttemptStatus::Expired;

                assert!(attempt.is_finished());
            }

            mod edge_cases {
                use super::*;

                #[test]
                fn returns_false_when_status_is_requested() {
                    let attempt = struct_valid();

                    assert!(!attempt.is_finished());
                }

                #[test]
                fn returns_false_when_status_is_pending() {
                    let mut attempt = struct_valid();
                    attempt.status = BoletoAttemptStatus::Pending;

                    assert!(!attempt.is_finished());
                }

                #[test]
                fn returns_false_when_status_is_cancellation_requested() {
                    let mut attempt = struct_valid();
                    attempt.status = BoletoAttemptStatus::CancellationRequested;

                    assert!(!attempt.is_finished());
                }
            }
        }
    }

    mod gateway_event_handler {
        use super::*;

        mod apply_event {
            use super::*;

            #[test]
            fn applies_paid_event_from_requested() {
                let mut attempt = struct_valid();

                let result = attempt.apply_event(BoletoGatewayEvent::Paid);

                assert!(matches!(result, GatewayAttemptEventResult::Applied));
                assert_eq!(attempt.status, BoletoAttemptStatus::Paid);
            }

            #[test]
            fn applies_cancelled_event_from_requested() {
                let mut attempt = struct_valid();

                let result = attempt.apply_event(BoletoGatewayEvent::Cancelled);

                assert!(matches!(result, GatewayAttemptEventResult::Applied));
                assert_eq!(attempt.status, BoletoAttemptStatus::Cancelled);
            }

            #[test]
            fn applies_expired_event_from_requested() {
                let mut attempt = struct_valid();

                let result = attempt.apply_event(BoletoGatewayEvent::Expired);

                assert!(matches!(result, GatewayAttemptEventResult::Applied));
                assert_eq!(attempt.status, BoletoAttemptStatus::Expired);
            }

            #[test]
            fn applies_failed_event_from_requested() {
                let mut attempt = struct_valid();

                let result = attempt.apply_event(BoletoGatewayEvent::Failed);

                assert!(matches!(result, GatewayAttemptEventResult::Applied));
                assert_eq!(attempt.status, BoletoAttemptStatus::Failed);
            }

            #[test]
            fn applies_paid_event_from_pending() {
                let mut attempt = struct_valid();
                attempt.status = BoletoAttemptStatus::Pending;

                let result = attempt.apply_event(BoletoGatewayEvent::Paid);

                assert!(matches!(result, GatewayAttemptEventResult::Applied));
                assert_eq!(attempt.status, BoletoAttemptStatus::Paid);
            }

            #[test]
            fn applies_paid_event_from_cancellation_requested() {
                let mut attempt = struct_valid();
                attempt.status = BoletoAttemptStatus::CancellationRequested;

                let result = attempt.apply_event(BoletoGatewayEvent::Paid);

                assert!(matches!(result, GatewayAttemptEventResult::Applied));
                assert_eq!(attempt.status, BoletoAttemptStatus::Paid);
            }

            mod edge_cases {
                use super::*;

                #[test]
                fn cannot_apply_paid_event_when_already_paid() {
                    let mut attempt = struct_valid();
                    attempt.status = BoletoAttemptStatus::Paid;

                    let result = attempt.apply_event(BoletoGatewayEvent::Paid);

                    assert!(matches!(result, GatewayAttemptEventResult::Conflict(_)));
                    assert_eq!(attempt.status, BoletoAttemptStatus::Paid);
                }

                #[test]
                fn cannot_apply_paid_event_when_already_cancelled() {
                    let mut attempt = struct_valid();
                    attempt.status = BoletoAttemptStatus::Cancelled;

                    let result = attempt.apply_event(BoletoGatewayEvent::Paid);

                    assert!(matches!(result, GatewayAttemptEventResult::Conflict(_)));
                    assert_eq!(attempt.status, BoletoAttemptStatus::Cancelled);
                }

                #[test]
                fn cannot_apply_paid_event_when_already_expired() {
                    let mut attempt = struct_valid();
                    attempt.status = BoletoAttemptStatus::Expired;

                    let result = attempt.apply_event(BoletoGatewayEvent::Paid);

                    assert!(matches!(result, GatewayAttemptEventResult::Conflict(_)));
                    assert_eq!(attempt.status, BoletoAttemptStatus::Expired);
                }

                #[test]
                fn cannot_apply_paid_event_when_already_failed() {
                    let mut attempt = struct_valid();
                    attempt.status = BoletoAttemptStatus::Failed;

                    let result = attempt.apply_event(BoletoGatewayEvent::Paid);

                    assert!(matches!(result, GatewayAttemptEventResult::Conflict(_)));
                    assert_eq!(attempt.status, BoletoAttemptStatus::Failed);
                }

                #[test]
                fn preserves_status_when_event_is_rejected() {
                    let mut attempt = struct_valid();
                    attempt.status = BoletoAttemptStatus::Cancelled;

                    let original_status = attempt.status;

                    let result = attempt.apply_event(BoletoGatewayEvent::Expired);

                    assert!(matches!(result, GatewayAttemptEventResult::Conflict(_)));
                    assert_eq!(attempt.status, original_status);
                }

                #[test]
                fn conflict_result_references_correct_attempt_id() {
                    let mut attempt = struct_valid();
                    attempt.status = BoletoAttemptStatus::Paid;
                    let expected_id = attempt.id();

                    let result = attempt.apply_event(BoletoGatewayEvent::Paid);

                    match result {
                        GatewayAttemptEventResult::Conflict(conflict) => {
                            let (id, event) = conflict.into_parts();
                            assert_eq!(id, expected_id);
                            assert!(matches!(
                                event,
                                GatewayEvent::Boleto(BoletoGatewayEvent::Paid)
                            ));
                        }
                        _ => panic!("expected a conflict result"),
                    }
                }

                #[test]
                fn reapplying_paid_event_after_success_returns_conflict() {
                    let mut attempt = struct_valid();

                    let first_result = attempt.apply_event(BoletoGatewayEvent::Paid);
                    assert!(matches!(first_result, GatewayAttemptEventResult::Applied));

                    let status_after_first_event = attempt.status;

                    let second_result = attempt.apply_event(BoletoGatewayEvent::Paid);

                    assert!(matches!(
                        second_result,
                        GatewayAttemptEventResult::Conflict(_)
                    ));
                    assert_eq!(attempt.status, status_after_first_event);
                }
            }
        }
    }
}


