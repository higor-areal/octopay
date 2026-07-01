use crate::value_objects::payment_method::outcome::{
    boleto::BoletoData, 
    card::CardData, 
    pix::PixData
};


pub enum PaymentMethodOutcome{
    Pix(PixData),
    Card(CardData),
    Boleto(BoletoData),
}