use std::{matches};

use crate::{entities::{contracts::{GatewayAttemptActions, GatewayAttemptState, GatewayEventHandler}, events::{GatewayEvent, PixGatewayEvent}, models::gateway_attempt::{GatewayAttemptConflict, GatewayAttemptEventResult}}, gateway::gateway_name::GatewayName, value_objects::{ids::{gateway_attempt_id::GatewayAttemptId, payment_attempt_id::PaymentAttemptId}, payment_method::outcome::pix::PixData}};



#[allow(dead_code)]
pub struct PixGatewayAttempt {
    id: GatewayAttemptId,
    payment_attempt_id: PaymentAttemptId,
    gateway_name: GatewayName,
    outcome: PixData,
    status: PixAttemptStatus,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixAttemptStatus {
    Requested,
    Pending,
    CancellationRequested,
    Paid, //
    Cancelled, //
    Expired, //
    Failed, //
}

impl PixGatewayAttempt {


    pub fn new(
        id: GatewayAttemptId,
        payment_attempt_id: PaymentAttemptId,
        gateway_name: GatewayName,
        outcome: PixData
    ) -> Self{
        let status = PixAttemptStatus::Requested;
        PixGatewayAttempt{
            id,
            payment_attempt_id,
            gateway_name,
            outcome,
            status
        }
    }

    pub fn status(&self) -> PixAttemptStatus {
        self.status
    }
    
    pub(crate) fn id(&self) -> GatewayAttemptId {
        self.id
    }
}


impl GatewayAttemptActions for PixGatewayAttempt{
    fn can_be_cancelled(&self) -> bool {
        matches!(
            self.status, 
            | PixAttemptStatus::Pending
        )
    }

    fn blocks_payment_attempt(&self) -> bool {
        matches!(
            self.status, 
            PixAttemptStatus::Paid
        )
    }
    fn request_cancellation(&mut self) -> bool {
        if !self.can_be_cancelled() {
            return false;
        }
        self.status = PixAttemptStatus::CancellationRequested;

        true
    }
}

impl GatewayAttemptState for PixGatewayAttempt{
    fn is_paid(&self) -> bool {
        matches!(
            self.status, 
            PixAttemptStatus::Paid
        )
    }

    fn is_failed(&self) -> bool {
        matches!(
            self.status, 
            PixAttemptStatus::Failed
        )
    }

    fn is_cancelled(&self) -> bool {
        matches!(
            self.status, 
            PixAttemptStatus::Cancelled
        )
    }

    fn is_finished(&self) -> bool {
        matches!(
            self.status, 
            PixAttemptStatus::Paid
            | PixAttemptStatus::Failed
            | PixAttemptStatus::Cancelled
            | PixAttemptStatus::Expired
        )
    }
}

impl GatewayEventHandler for PixGatewayAttempt {
    type Event = PixGatewayEvent;

    fn apply_event(
        &mut self,
        event: Self::Event,
    ) -> GatewayAttemptEventResult {
        match self.next_state(event) {
            Ok(status) => {
                self.status = status;
                GatewayAttemptEventResult::Applied
            }
            Err(conflict) => {
                    GatewayAttemptEventResult::Conflict( conflict )
            }
        }
    }
}

impl PixGatewayAttempt {

    fn next_state(
        &self,
        event: PixGatewayEvent,
    ) -> Result<PixAttemptStatus, GatewayAttemptConflict> {
        match (self.status, &event) {

            (
                PixAttemptStatus::Pending
                | PixAttemptStatus::Requested
                | PixAttemptStatus::CancellationRequested,
                PixGatewayEvent::Paid
            ) => {
                Ok(PixAttemptStatus::Paid)
            }

            // Estados que podem receber Failed
            (
                PixAttemptStatus::Pending
                | PixAttemptStatus::Requested,
                PixGatewayEvent::Failed,
            ) => {
                Ok(PixAttemptStatus::Failed)
            }

            // Cancelamento confirmado
            (
                PixAttemptStatus::CancellationRequested
                | PixAttemptStatus::Pending
                | PixAttemptStatus::Requested,
                PixGatewayEvent::Cancelled,
            ) => {
                Ok(PixAttemptStatus::Cancelled)
            }

            // Expiração natural
            (
                PixAttemptStatus::Pending
                | PixAttemptStatus::Requested
                | PixAttemptStatus::CancellationRequested,
                PixGatewayEvent::Expired,
            ) => {
                Ok(PixAttemptStatus::Expired)
            }

            _ => {
                Err(GatewayAttemptConflict::new(
                            self.id,
                            GatewayEvent::Pix(event),
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

    fn struct_valid() -> PixGatewayAttempt {
        let id = GatewayAttemptId::generate();
        let payment_attempt_id = PaymentAttemptId::generate();
        let gateway_name = GatewayName::MercadoPago;
        let outcome = PixData::new(
            "qr_code",
            None::<String>,
            None::<String>,
            Expiration::new(Utc::now()),
        );

        PixGatewayAttempt::new(
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
                attempt.status =PixAttemptStatus::Pending;

                assert!(attempt.can_be_cancelled());
            }

            mod edge_cases {
                use super::*;

                #[test]
                fn returns_false_when_status_is_requested() {
                    let mut attempt = struct_valid();
                    attempt.status = PixAttemptStatus::Requested;

                    assert!(!attempt.can_be_cancelled());
                }

                #[test]
                fn returns_false_when_status_is_cancellation_requested() {
                    let mut attempt = struct_valid();
                    attempt.status = PixAttemptStatus::CancellationRequested;

                    assert!(!attempt.can_be_cancelled());
                }

                #[test]
                fn returns_false_when_status_is_paid() {
                    let mut attempt = struct_valid();
                    attempt.status = PixAttemptStatus::Paid;

                    assert!(!attempt.can_be_cancelled());
                }

                #[test]
                fn returns_false_when_status_is_cancelled() {
                    let mut attempt = struct_valid();
                    attempt.status = PixAttemptStatus::Cancelled;

                    assert!(!attempt.can_be_cancelled());
                }

                #[test]
                fn returns_false_when_status_is_expired() {
                    let mut attempt = struct_valid();
                    attempt.status = PixAttemptStatus::Expired;

                    assert!(!attempt.can_be_cancelled());
                }

                #[test]
                fn returns_false_when_status_is_failed() {
                    let mut attempt = struct_valid();
                    attempt.status = PixAttemptStatus::Failed;

                    assert!(!attempt.can_be_cancelled());
                }
            }
        }

        mod blocks_payment_attempt {
            use super::*;

            #[test]
            fn returns_true_when_status_is_paid() {
                let mut attempt = struct_valid();
                attempt.status = PixAttemptStatus::Paid;

                assert!(attempt.blocks_payment_attempt());
            }

            mod edge_cases {
                use super::*;

                #[test]
                fn returns_false_when_status_is_pending() {
                    let attempt = struct_valid();

                    assert!(!attempt.blocks_payment_attempt());
                }

                #[test]
                fn returns_false_when_status_is_requested() {
                    let mut attempt = struct_valid();
                    attempt.status = PixAttemptStatus::Requested;

                    assert!(!attempt.blocks_payment_attempt());
                }

                #[test]
                fn returns_false_when_status_is_cancellation_requested() {
                    let mut attempt = struct_valid();
                    attempt.status = PixAttemptStatus::CancellationRequested;

                    assert!(!attempt.blocks_payment_attempt());
                }

                #[test]
                fn returns_false_when_status_is_cancelled() {
                    let mut attempt = struct_valid();
                    attempt.status = PixAttemptStatus::Cancelled;

                    assert!(!attempt.blocks_payment_attempt());
                }

                #[test]
                fn returns_false_when_status_is_expired() {
                    let mut attempt = struct_valid();
                    attempt.status = PixAttemptStatus::Expired;

                    assert!(!attempt.blocks_payment_attempt());
                }

                #[test]
                fn returns_false_when_status_is_failed() {
                    let mut attempt = struct_valid();
                    attempt.status = PixAttemptStatus::Failed;

                    assert!(!attempt.blocks_payment_attempt());
                }
            }
        }

        mod request_cancellation {
            use super::*;

            #[test]
            fn cancels_and_returns_true_when_status_is_pending() {
                let mut attempt = struct_valid();
                attempt.status = PixAttemptStatus::Pending;

                let result = attempt.request_cancellation();

                assert!(result);
                assert_eq!(attempt.status(), PixAttemptStatus::CancellationRequested);
            }

            mod edge_cases {
                use super::*;

                #[test]
                fn returns_false_and_preserves_status_when_requested() {
                    let mut attempt = struct_valid();
                    attempt.status = PixAttemptStatus::Requested;

                    let result = attempt.request_cancellation();

                    assert!(!result);
                    assert_eq!(attempt.status(), PixAttemptStatus::Requested);
                }

                #[test]
                fn returns_false_andlrea_preserves_status_when_cancellation_ady_requested() {
                    let mut attempt = struct_valid();
                    attempt.status = PixAttemptStatus::CancellationRequested;

                    let result = attempt.request_cancellation();

                    assert!(!result);
                    assert_eq!(attempt.status(), PixAttemptStatus::CancellationRequested);
                }

                #[test]
                fn returns_false_and_preserves_status_when_paid() {
                    let mut attempt = struct_valid();
                    attempt.status = PixAttemptStatus::Paid;

                    let result = attempt.request_cancellation();

                    assert!(!result);
                    assert_eq!(attempt.status(), PixAttemptStatus::Paid);
                }

                #[test]
                fn returns_false_and_preserves_status_when_cancelled() {
                    let mut attempt = struct_valid();
                    attempt.status = PixAttemptStatus::Cancelled;

                    let result = attempt.request_cancellation();

                    assert!(!result);
                    assert_eq!(attempt.status(), PixAttemptStatus::Cancelled);
                }

                #[test]
                fn returns_false_and_preserves_status_when_expired() {
                    let mut attempt = struct_valid();
                    attempt.status = PixAttemptStatus::Expired;

                    let result = attempt.request_cancellation();

                    assert!(!result);
                    assert_eq!(attempt.status(), PixAttemptStatus::Expired);
                }

                #[test]
                fn returns_false_and_preserves_status_when_failed() {
                    let mut attempt = struct_valid();
                    attempt.status = PixAttemptStatus::Failed;

                    let result = attempt.request_cancellation();

                    assert!(!result);
                    assert_eq!(attempt.status(), PixAttemptStatus::Failed);
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
                attempt.status = PixAttemptStatus::Paid;

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
                attempt.status = PixAttemptStatus::Failed;

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
                attempt.status = PixAttemptStatus::Cancelled;

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
                attempt.status = PixAttemptStatus::Paid;

                assert!(attempt.is_finished());
            }

            #[test]
            fn returns_true_when_status_is_failed() {
                let mut attempt = struct_valid();
                attempt.status = PixAttemptStatus::Failed;

                assert!(attempt.is_finished());
            }

            #[test]
            fn returns_true_when_status_is_cancelled() {
                let mut attempt = struct_valid();
                attempt.status = PixAttemptStatus::Cancelled;

                assert!(attempt.is_finished());
            }

            #[test]
            fn returns_true_when_status_is_expired() {
                let mut attempt = struct_valid();
                attempt.status = PixAttemptStatus::Expired;

                assert!(attempt.is_finished());
            }

            mod edge_cases {
                use super::*;

                #[test]
                fn returns_false_when_status_is_pending() {
                    let attempt = struct_valid();

                    assert!(!attempt.is_finished());
                }

                #[test]
                fn returns_false_when_status_is_requested() {
                    let mut attempt = struct_valid();
                    attempt.status = PixAttemptStatus::Requested;

                    assert!(!attempt.is_finished());
                }

                #[test]
                fn returns_false_when_status_is_cancellation_requested() {
                    let mut attempt = struct_valid();
                    attempt.status = PixAttemptStatus::CancellationRequested;

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
            fn applies_paid_event_from_pending() {
                let mut attempt = struct_valid();

                let result = attempt.apply_event(PixGatewayEvent::Paid);

                assert!(matches!(result, GatewayAttemptEventResult::Applied));
                assert_eq!(attempt.status(), PixAttemptStatus::Paid);
            }

            #[test]
            fn applies_paid_event_from_requested() {
                let mut attempt = struct_valid();
                attempt.status = PixAttemptStatus::Requested;

                let result = attempt.apply_event(PixGatewayEvent::Paid);

                assert!(matches!(result, GatewayAttemptEventResult::Applied));
                assert_eq!(attempt.status(), PixAttemptStatus::Paid);
            }

            #[test]
            fn applies_paid_event_from_cancellation_requested() {
                let mut attempt = struct_valid();
                attempt.status = PixAttemptStatus::CancellationRequested;

                let result = attempt.apply_event(PixGatewayEvent::Paid);

                assert!(matches!(result, GatewayAttemptEventResult::Applied));
                assert_eq!(attempt.status(), PixAttemptStatus::Paid);
            }

            #[test]
            fn applies_failed_event_from_pending() {
                let mut attempt = struct_valid();

                let result = attempt.apply_event(PixGatewayEvent::Failed);

                assert!(matches!(result, GatewayAttemptEventResult::Applied));
                assert_eq!(attempt.status(), PixAttemptStatus::Failed);
            }

            #[test]
            fn applies_failed_event_from_requested() {
                let mut attempt = struct_valid();
                attempt.status = PixAttemptStatus::Requested;

                let result = attempt.apply_event(PixGatewayEvent::Failed);

                assert!(matches!(result, GatewayAttemptEventResult::Applied));
                assert_eq!(attempt.status(), PixAttemptStatus::Failed);
            }

            #[test]
            fn applies_cancelled_event_from_pending() {
                let mut attempt = struct_valid();

                let result = attempt.apply_event(PixGatewayEvent::Cancelled);

                assert!(matches!(result, GatewayAttemptEventResult::Applied));
                assert_eq!(attempt.status(), PixAttemptStatus::Cancelled);
            }

            #[test]
            fn applies_cancelled_event_from_requested() {
                let mut attempt = struct_valid();
                attempt.status = PixAttemptStatus::Requested;

                let result = attempt.apply_event(PixGatewayEvent::Cancelled);

                assert!(matches!(result, GatewayAttemptEventResult::Applied));
                assert_eq!(attempt.status(), PixAttemptStatus::Cancelled);
            }

            #[test]
            fn applies_cancelled_event_from_cancellation_requested() {
                let mut attempt = struct_valid();
                attempt.status = PixAttemptStatus::CancellationRequested;

                let result = attempt.apply_event(PixGatewayEvent::Cancelled);

                assert!(matches!(result, GatewayAttemptEventResult::Applied));
                assert_eq!(attempt.status(), PixAttemptStatus::Cancelled);
            }

            #[test]
            fn applies_expired_event_from_pending() {
                let mut attempt = struct_valid();

                let result = attempt.apply_event(PixGatewayEvent::Expired);

                assert!(matches!(result, GatewayAttemptEventResult::Applied));
                assert_eq!(attempt.status(), PixAttemptStatus::Expired);
            }

            #[test]
            fn applies_expired_event_from_requested() {
                let mut attempt = struct_valid();
                attempt.status = PixAttemptStatus::Requested;

                let result = attempt.apply_event(PixGatewayEvent::Expired);

                assert!(matches!(result, GatewayAttemptEventResult::Applied));
                assert_eq!(attempt.status(), PixAttemptStatus::Expired);
            }

            #[test]
            fn applies_expired_event_from_cancellation_requested() {
                let mut attempt = struct_valid();
                attempt.status = PixAttemptStatus::CancellationRequested;

                let result = attempt.apply_event(PixGatewayEvent::Expired);

                assert!(matches!(result, GatewayAttemptEventResult::Applied));
                assert_eq!(attempt.status(), PixAttemptStatus::Expired);
            }

            mod edge_cases {
                use super::*;

                // Estado terminal (Paid) rejeita todos os eventos.
                #[test]
                fn cannot_apply_paid_event_when_already_paid() {
                    let mut attempt = struct_valid();
                    attempt.status = PixAttemptStatus::Paid;

                    let result = attempt.apply_event(PixGatewayEvent::Paid);

                    assert!(matches!(result, GatewayAttemptEventResult::Conflict(_)));
                    assert_eq!(attempt.status(), PixAttemptStatus::Paid);
                }

                #[test]
                fn cannot_apply_failed_event_when_already_paid() {
                    let mut attempt = struct_valid();
                    attempt.status = PixAttemptStatus::Paid;

                    let result = attempt.apply_event(PixGatewayEvent::Failed);

                    assert!(matches!(result, GatewayAttemptEventResult::Conflict(_)));
                    assert_eq!(attempt.status(), PixAttemptStatus::Paid);
                }

                #[test]
                fn cannot_apply_cancelled_event_when_already_paid() {
                    let mut attempt = struct_valid();
                    attempt.status = PixAttemptStatus::Paid;

                    let result = attempt.apply_event(PixGatewayEvent::Cancelled);

                    assert!(matches!(result, GatewayAttemptEventResult::Conflict(_)));
                    assert_eq!(attempt.status(), PixAttemptStatus::Paid);
                }

                #[test]
                fn cannot_apply_expired_event_when_already_paid() {
                    let mut attempt = struct_valid();
                    attempt.status = PixAttemptStatus::Paid;

                    let result = attempt.apply_event(PixGatewayEvent::Expired);

                    assert!(matches!(result, GatewayAttemptEventResult::Conflict(_)));
                    assert_eq!(attempt.status(), PixAttemptStatus::Paid);
                }

                // Demais estados terminais, representados por um evento (Paid).
                #[test]
                fn cannot_apply_paid_event_when_already_cancelled() {
                    let mut attempt = struct_valid();
                    attempt.status = PixAttemptStatus::Cancelled;

                    let result = attempt.apply_event(PixGatewayEvent::Paid);

                    assert!(matches!(result, GatewayAttemptEventResult::Conflict(_)));
                    assert_eq!(attempt.status(), PixAttemptStatus::Cancelled);
                }

                #[test]
                fn cannot_apply_paid_event_when_already_expired() {
                    let mut attempt = struct_valid();
                    attempt.status = PixAttemptStatus::Expired;

                    let result = attempt.apply_event(PixGatewayEvent::Paid);

                    assert!(matches!(result, GatewayAttemptEventResult::Conflict(_)));
                    assert_eq!(attempt.status(), PixAttemptStatus::Expired);
                }

                #[test]
                fn cannot_apply_paid_event_when_already_failed() {
                    let mut attempt = struct_valid();
                    attempt.status = PixAttemptStatus::Failed;

                    let result = attempt.apply_event(PixGatewayEvent::Paid);

                    assert!(matches!(result, GatewayAttemptEventResult::Conflict(_)));
                    assert_eq!(attempt.status(), PixAttemptStatus::Failed);
                }

                // Caso assimétrico relevante: CancellationRequested aceita
                // Paid/Cancelled/Expired, mas rejeita Failed.
                #[test]
                fn cannot_apply_failed_event_when_cancellation_requested() {
                    let mut attempt = struct_valid();
                    attempt.status = PixAttemptStatus::CancellationRequested;

                    let result = attempt.apply_event(PixGatewayEvent::Failed);

                    assert!(matches!(result, GatewayAttemptEventResult::Conflict(_)));
                    assert_eq!(attempt.status(), PixAttemptStatus::CancellationRequested);
                }

                #[test]
                fn preserves_status_when_event_is_rejected() {
                    let mut attempt = struct_valid();
                    attempt.status = PixAttemptStatus::Cancelled;

                    let original_status = attempt.status();

                    let result = attempt.apply_event(PixGatewayEvent::Expired);

                    assert!(matches!(result, GatewayAttemptEventResult::Conflict(_)));
                    assert_eq!(attempt.status(), original_status);
                }

                #[test]
                fn conflict_result_references_correct_attempt_id() {
                    let mut attempt = struct_valid();
                    attempt.status = PixAttemptStatus::Paid;
                    let expected_id = attempt.id();

                    let result = attempt.apply_event(PixGatewayEvent::Paid);

                    match result {
                        GatewayAttemptEventResult::Conflict(conflict) => {
                            let (id, event) = conflict.into_parts();
                            assert_eq!(id, expected_id);
                            assert!(matches!(
                                event,
                                GatewayEvent::Pix(PixGatewayEvent::Paid)
                            ));
                        }
                        _ => panic!("expected a conflict result"),
                    }
                }

                #[test]
                fn reapplying_paid_event_after_success_returns_conflict() {
                    let mut attempt = struct_valid();

                    let first_result = attempt.apply_event(PixGatewayEvent::Paid);
                    assert!(matches!(first_result, GatewayAttemptEventResult::Applied));

                    let status_after_first_event = attempt.status();

                    let second_result = attempt.apply_event(PixGatewayEvent::Paid);

                    assert!(matches!(
                        second_result,
                        GatewayAttemptEventResult::Conflict(_)
                    ));
                    assert_eq!(attempt.status(), status_after_first_event);
                }
            }
        }
    }
}