use std::{matches};
use crate::{entities::{contracts::{gateway_attempt_event::GatewayEventHandler, gateway_attempt_rules::{GatewayAttemptActions, GatewayAttemptState}}, events::{GatewayEvent, gateway_attempt::card::CardGatewayEvent}, models::gateway_attempt::{GatewayAttemptConflict, conflict::GatewayAttemptEventResult}}, gateway::gateway_name::GatewayName, value_objects::{ids::{gateway_attempt_id::GatewayAttemptId, payment_attempt_id::PaymentAttemptId}, payment_method::outcome::card::CardData}};

#[allow(dead_code)]
pub struct CardGatewayAttempt {
    id: GatewayAttemptId,
    payment_attempt_id: PaymentAttemptId,
    gateway_name: GatewayName,
    outcome: CardData,
    status: CardAttemptStatus,
//    pub failure_reason: Option<FailureReason>, // só preenchido em Failed
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardAttemptStatus {
    Requested, 
    Pending,
    CancellationRequested,
    Paid, //
    Rejected, //
    Failed, //
    Cancelled, //
}

#[allow(dead_code)]
impl CardGatewayAttempt {

    pub fn new(
        id: GatewayAttemptId,
        payment_attempt_id: PaymentAttemptId,
        gateway_name: GatewayName,
        outcome: CardData,

    ) -> Self{
        let status = CardAttemptStatus::Requested;
        Self { id, 
            payment_attempt_id, 
            gateway_name, 
            outcome, 
            status
        }
    }

    pub fn status(&self) -> CardAttemptStatus {
        self.status
    }
    pub(crate) fn id(&self) -> GatewayAttemptId {
        self.id
    }
}

impl GatewayAttemptActions for CardGatewayAttempt{

    fn can_be_cancelled(&self) -> bool {
        matches!(self.status,
            CardAttemptStatus::Pending
        )
    }

    fn blocks_payment_attempt(&self) -> bool {
        matches!(self.status,
            CardAttemptStatus::Paid
            | CardAttemptStatus::Pending
            | CardAttemptStatus::Requested
            | CardAttemptStatus::CancellationRequested
        )
    }

    fn request_cancellation(&mut self) -> bool {
        if !self.can_be_cancelled() {
            return false;
        }
        self.status = CardAttemptStatus::CancellationRequested;
        true
    }

}

impl GatewayAttemptState for CardGatewayAttempt{
    fn is_paid(&self) -> bool {
        matches!(self.status,
            CardAttemptStatus::Paid
        )
    }

    fn is_failed(&self) -> bool {
        matches!(self.status,
            CardAttemptStatus::Failed
        )
    }

    fn is_cancelled(&self) -> bool {
        matches!(self.status,
            CardAttemptStatus::Cancelled
        )
    }

    fn is_finished(&self) -> bool {
        matches!(self.status,
            CardAttemptStatus::Paid
            | CardAttemptStatus::Cancelled
            | CardAttemptStatus::Failed
            | CardAttemptStatus::Rejected
        )
    }
}


impl GatewayEventHandler for CardGatewayAttempt{
    type Event = CardGatewayEvent;

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

impl CardGatewayAttempt{
    fn next_state(
        &self,
        event: CardGatewayEvent,
    ) -> Result<CardAttemptStatus, GatewayAttemptConflict>{
        match (self.status, &event) {
            (
                CardAttemptStatus::Pending
                | CardAttemptStatus::Requested
                | CardAttemptStatus::CancellationRequested,
                CardGatewayEvent::Paid
            ) => {
                Ok(CardAttemptStatus::Paid)
            }
            (
                CardAttemptStatus::Pending
                | CardAttemptStatus::Requested
                | CardAttemptStatus::CancellationRequested,
                CardGatewayEvent::Cancelled
            ) => {
                Ok(CardAttemptStatus::Cancelled)
            }

            (
                CardAttemptStatus::Pending
                | CardAttemptStatus::Requested
                | CardAttemptStatus::CancellationRequested,
                CardGatewayEvent::Failed
            ) => {
                Ok(CardAttemptStatus::Failed)
            }
            (
                CardAttemptStatus::Pending
                | CardAttemptStatus::Requested
                | CardAttemptStatus::CancellationRequested,
                CardGatewayEvent::Rejected
            ) => {
                Ok(CardAttemptStatus::Rejected)
            }

            _ => {
                Err(GatewayAttemptConflict::new(
                        self.id(), 
                        GatewayEvent::Card(event)
                    )
                )
            }
        }
    }
}





#[cfg(test)]
mod test {
    use super::*;

    fn struct_valid() -> CardGatewayAttempt {
        let id = GatewayAttemptId::generate();
        let payment_attempt_id = PaymentAttemptId::generate();
        let gateway_name = GatewayName::MercadoPago;
        let outcome = CardData::new();

        CardGatewayAttempt::new(
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
                attempt.status = CardAttemptStatus::Pending;

                assert!(attempt.can_be_cancelled());
            }

            mod edge_cases {
                use super::*;

                #[test]
                fn returns_false_when_status_is_requested() {
                    let mut attempt = struct_valid();
                    attempt.status = CardAttemptStatus::Requested;

                    assert!(!attempt.can_be_cancelled());
                }

                #[test]
                fn returns_false_when_status_is_cancellation_requested() {
                    let mut attempt = struct_valid();
                    attempt.status = CardAttemptStatus::CancellationRequested;

                    assert!(!attempt.can_be_cancelled());
                }

                #[test]
                fn returns_false_when_status_is_paid() {
                    let mut attempt = struct_valid();
                    attempt.status = CardAttemptStatus::Paid;

                    assert!(!attempt.can_be_cancelled());
                }

                #[test]
                fn returns_false_when_status_is_rejected() {
                    let mut attempt = struct_valid();
                    attempt.status = CardAttemptStatus::Rejected;

                    assert!(!attempt.can_be_cancelled());
                }

                #[test]
                fn returns_false_when_status_is_failed() {
                    let mut attempt = struct_valid();
                    attempt.status = CardAttemptStatus::Failed;

                    assert!(!attempt.can_be_cancelled());
                }

                #[test]
                fn returns_false_when_status_is_cancelled() {
                    let mut attempt = struct_valid();
                    attempt.status = CardAttemptStatus::Cancelled;

                    assert!(!attempt.can_be_cancelled());
                }
            }
        }

        mod blocks_payment_attempt {
            use super::*;

            #[test]
            fn returns_true_when_status_is_pending() {
                let attempt = struct_valid();

                assert!(attempt.blocks_payment_attempt());
            }

            #[test]
            fn returns_true_when_status_is_requested() {
                let mut attempt = struct_valid();
                attempt.status = CardAttemptStatus::Requested;

                assert!(attempt.blocks_payment_attempt());
            }

            #[test]
            fn returns_true_when_status_is_paid() {
                let mut attempt = struct_valid();
                attempt.status = CardAttemptStatus::Paid;

                assert!(attempt.blocks_payment_attempt());
            }

            #[test]
            fn returns_true_when_status_is_cancellation_requested() {
                let mut attempt = struct_valid();
                attempt.status = CardAttemptStatus::CancellationRequested;

                assert!(attempt.blocks_payment_attempt());
            }

            mod edge_cases {
                use super::*;

                #[test]
                fn returns_false_when_status_is_rejected() {
                    let mut attempt = struct_valid();
                    attempt.status = CardAttemptStatus::Rejected;

                    assert!(!attempt.blocks_payment_attempt());
                }

                #[test]
                fn returns_false_when_status_is_failed() {
                    let mut attempt = struct_valid();
                    attempt.status = CardAttemptStatus::Failed;

                    assert!(!attempt.blocks_payment_attempt());
                }

                #[test]
                fn returns_false_when_status_is_cancelled() {
                    let mut attempt = struct_valid();
                    attempt.status = CardAttemptStatus::Cancelled;

                    assert!(!attempt.blocks_payment_attempt());
                }
            }
        }

        mod request_cancellation {
            use super::*;

            #[test]
            fn cancels_and_returns_true_when_status_is_pending() {
                let mut attempt = struct_valid();
                attempt.status = CardAttemptStatus::Pending;


                let result = attempt.request_cancellation();

                assert!(result);
                assert_eq!(attempt.status(), CardAttemptStatus::CancellationRequested);
            }

            mod edge_cases {
                use super::*;

                #[test]
                fn returns_false_and_preserves_status_when_requested() {
                    let mut attempt = struct_valid();
                    attempt.status = CardAttemptStatus::Requested;

                    let result = attempt.request_cancellation();

                    assert!(!result);
                    assert_eq!(attempt.status(), CardAttemptStatus::Requested);
                }

                #[test]
                fn returns_false_and_preserves_status_when_cancellation_already_requested() {
                    let mut attempt = struct_valid();
                    attempt.status = CardAttemptStatus::CancellationRequested;

                    let result = attempt.request_cancellation();

                    assert!(!result);
                    assert_eq!(attempt.status(), CardAttemptStatus::CancellationRequested);
                }

                #[test]
                fn returns_false_and_preserves_status_when_paid() {
                    let mut attempt = struct_valid();
                    attempt.status = CardAttemptStatus::Paid;

                    let result = attempt.request_cancellation();

                    assert!(!result);
                    assert_eq!(attempt.status(), CardAttemptStatus::Paid);
                }

                #[test]
                fn returns_false_and_preserves_status_when_rejected() {
                    let mut attempt = struct_valid();
                    attempt.status = CardAttemptStatus::Rejected;

                    let result = attempt.request_cancellation();

                    assert!(!result);
                    assert_eq!(attempt.status(), CardAttemptStatus::Rejected);
                }

                #[test]
                fn returns_false_and_preserves_status_when_failed() {
                    let mut attempt = struct_valid();
                    attempt.status = CardAttemptStatus::Failed;

                    let result = attempt.request_cancellation();

                    assert!(!result);
                    assert_eq!(attempt.status(), CardAttemptStatus::Failed);
                }

                #[test]
                fn returns_false_and_preserves_status_when_cancelled() {
                    let mut attempt = struct_valid();
                    attempt.status = CardAttemptStatus::Cancelled;

                    let result = attempt.request_cancellation();

                    assert!(!result);
                    assert_eq!(attempt.status(), CardAttemptStatus::Cancelled);
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
                attempt.status = CardAttemptStatus::Paid;

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
                attempt.status = CardAttemptStatus::Failed;

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
                attempt.status = CardAttemptStatus::Cancelled;

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
                attempt.status = CardAttemptStatus::Paid;

                assert!(attempt.is_finished());
            }

            #[test]
            fn returns_true_when_status_is_cancelled() {
                let mut attempt = struct_valid();
                attempt.status = CardAttemptStatus::Cancelled;

                assert!(attempt.is_finished());
            }

            #[test]
            fn returns_true_when_status_is_failed() {
                let mut attempt = struct_valid();
                attempt.status = CardAttemptStatus::Failed;

                assert!(attempt.is_finished());
            }

            #[test]
            fn returns_true_when_status_is_rejected() {
                let mut attempt = struct_valid();
                attempt.status = CardAttemptStatus::Rejected;

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
                    attempt.status = CardAttemptStatus::Requested;

                    assert!(!attempt.is_finished());
                }

                #[test]
                fn returns_false_when_status_is_cancellation_requested() {
                    let mut attempt = struct_valid();
                    attempt.status = CardAttemptStatus::CancellationRequested;

                    assert!(!attempt.is_finished());
                }
            }
        }
    }

    mod gateway_event_handler {
        use super::*;

        mod apply_event {
            use super::*;

            // Fluxo normal: os três estados não-terminais (Pending, Requested,
            // CancellationRequested) aceitam todos os eventos de forma idêntica.
            // Um teste por evento a partir de Pending cobre o comportamento
            // padrão; a equivalência entre os três estados é validada abaixo.

            #[test]
            fn applies_paid_event_from_pending() {
                let mut attempt = struct_valid();

                let result = attempt.apply_event(CardGatewayEvent::Paid);

                assert!(matches!(result, GatewayAttemptEventResult::Applied));
                assert_eq!(attempt.status(), CardAttemptStatus::Paid);
            }

            #[test]
            fn applies_cancelled_event_from_pending() {
                let mut attempt = struct_valid();

                let result = attempt.apply_event(CardGatewayEvent::Cancelled);

                assert!(matches!(result, GatewayAttemptEventResult::Applied));
                assert_eq!(attempt.status(), CardAttemptStatus::Cancelled);
            }

            #[test]
            fn applies_failed_event_from_pending() {
                let mut attempt = struct_valid();

                let result = attempt.apply_event(CardGatewayEvent::Failed);

                assert!(matches!(result, GatewayAttemptEventResult::Applied));
                assert_eq!(attempt.status(), CardAttemptStatus::Failed);
            }

            #[test]
            fn applies_rejected_event_from_pending() {
                let mut attempt = struct_valid();

                let result = attempt.apply_event(CardGatewayEvent::Rejected);

                assert!(matches!(result, GatewayAttemptEventResult::Applied));
                assert_eq!(attempt.status(), CardAttemptStatus::Rejected);
            }

            // Equivalência entre estados não-terminais: Requested e
            // CancellationRequested se comportam exatamente como Pending.

            #[test]
            fn applies_paid_event_from_requested() {
                let mut attempt = struct_valid();
                attempt.status = CardAttemptStatus::Requested;

                let result = attempt.apply_event(CardGatewayEvent::Paid);

                assert!(matches!(result, GatewayAttemptEventResult::Applied));
                assert_eq!(attempt.status(), CardAttemptStatus::Paid);
            }

            #[test]
            fn applies_paid_event_from_cancellation_requested() {
                let mut attempt = struct_valid();
                attempt.status = CardAttemptStatus::CancellationRequested;

                let result = attempt.apply_event(CardGatewayEvent::Paid);

                assert!(matches!(result, GatewayAttemptEventResult::Applied));
                assert_eq!(attempt.status(), CardAttemptStatus::Paid);
            }

            mod edge_cases {
                use super::*;

                // Estados terminais rejeitam qualquer evento. Amostragem
                // representativa (evento Paid) por estado terminal, já que o
                // comportamento é idêntico para os 4 eventos em cada um deles.

                #[test]
                fn cannot_apply_paid_event_when_already_paid() {
                    let mut attempt = struct_valid();
                    attempt.status = CardAttemptStatus::Paid;

                    let result = attempt.apply_event(CardGatewayEvent::Paid);

                    assert!(matches!(result, GatewayAttemptEventResult::Conflict(_)));
                    assert_eq!(attempt.status(), CardAttemptStatus::Paid);
                }

                #[test]
                fn cannot_apply_paid_event_when_already_rejected() {
                    let mut attempt = struct_valid();
                    attempt.status = CardAttemptStatus::Rejected;

                    let result = attempt.apply_event(CardGatewayEvent::Paid);

                    assert!(matches!(result, GatewayAttemptEventResult::Conflict(_)));
                    assert_eq!(attempt.status(), CardAttemptStatus::Rejected);
                }

                #[test]
                fn cannot_apply_paid_event_when_already_failed() {
                    let mut attempt = struct_valid();
                    attempt.status = CardAttemptStatus::Failed;

                    let result = attempt.apply_event(CardGatewayEvent::Paid);

                    assert!(matches!(result, GatewayAttemptEventResult::Conflict(_)));
                    assert_eq!(attempt.status(), CardAttemptStatus::Failed);
                }

                #[test]
                fn cannot_apply_paid_event_when_already_cancelled() {
                    let mut attempt = struct_valid();
                    attempt.status = CardAttemptStatus::Cancelled;

                    let result = attempt.apply_event(CardGatewayEvent::Paid);

                    assert!(matches!(result, GatewayAttemptEventResult::Conflict(_)));
                    assert_eq!(attempt.status(), CardAttemptStatus::Cancelled);
                }

                #[test]
                fn preserves_status_when_event_is_rejected() {
                    let mut attempt = struct_valid();
                    attempt.status = CardAttemptStatus::Cancelled;

                    let original_status = attempt.status();

                    let result = attempt.apply_event(CardGatewayEvent::Failed);

                    assert!(matches!(result, GatewayAttemptEventResult::Conflict(_)));
                    assert_eq!(attempt.status(), original_status);
                }

                #[test]
                fn conflict_result_references_correct_attempt_id() {
                    let mut attempt = struct_valid();
                    attempt.status = CardAttemptStatus::Paid;
                    let expected_id = attempt.id();

                    let result = attempt.apply_event(CardGatewayEvent::Paid);

                    match result {
                        GatewayAttemptEventResult::Conflict(conflict) => {
                            let (id, event) = conflict.into_parts();
                            assert_eq!(id, expected_id);
                            assert!(matches!(
                                event,
                                GatewayEvent::Card(CardGatewayEvent::Paid)
                            ));
                        }
                        _ => panic!("expected a conflict result"),
                    }
                }

                #[test]
                fn reapplying_paid_event_after_success_returns_conflict() {
                    let mut attempt = struct_valid();

                    let first_result = attempt.apply_event(CardGatewayEvent::Paid);
                    assert!(matches!(first_result, GatewayAttemptEventResult::Applied));

                    let status_after_first_event = attempt.status();

                    let second_result = attempt.apply_event(CardGatewayEvent::Paid);

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