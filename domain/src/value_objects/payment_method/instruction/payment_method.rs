use crate::value_objects::payment_method::instruction::{
    pix::PixData, card::CardData, boleto::BoletoData
};


pub enum PaymentMethod{
    Pix(PixData),
    Card(CardData),
    Boleto(BoletoData),
}