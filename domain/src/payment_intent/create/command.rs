use crate::value_objects::{amount::Amount, payer::{document::Document, payer::Payer}, payment_method::instruction::payment_method::PaymentMethod};


#[allow(dead_code)]
pub struct Command{
    amount: Amount,
    payer: Payer,
    document: Document,
    payment_method: PaymentMethod 
}