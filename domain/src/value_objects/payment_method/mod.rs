pub mod instruction;
pub mod outcome;

//PixMethodId, CardMethodId, CardKind, BoletoMethodId

#[allow(dead_code)]
enum PixMethodId{
    Pix
}
#[allow(dead_code)]
pub enum CardKind {
    Visa,
    Mastercard,
    Elo,
    Amex,
    Hipercard,
    Unknown,
}
#[allow(dead_code)]
enum CardMethodId{
    Credit,
    Debit
}
#[allow(dead_code)]
enum BoletoMethodId{
    Boleto
}