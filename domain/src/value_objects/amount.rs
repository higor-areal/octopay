use crate::error::ValidationError;

#[allow(dead_code)]
pub struct Amount(u64);

impl Amount{

    pub const MIN: u64 = 100;
    pub const MAX: u64 = 100_000_000;

    pub fn new(value: u64) -> Result<Self, ValidationError>{
        
        Self::validation(value)?;
        Ok(
            Self(value)
        )
    }

    pub fn cents(&self) -> u64{
        self.0
    }

    pub fn currency(&self) -> String{
        let value = self.cents();

        let int: u64 = (value / 100) as u64;
        let cents: u64 = value % 100;

        format!("{}.{:02}", int, cents)
    }

    fn validation(value: u64) -> Result<(), ValidationError>{
        if value < Self::MIN {
            return Err(ValidationError::InvalidAmount);
        }

        if value > Self::MAX {
            return Err(ValidationError::InvalidAmount);
        }
        Ok(())
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_amount_when_value_is_equal_min() {
        let amount = Amount::new(Amount::MIN);

        assert!(amount.is_ok());
        assert_eq!(amount.unwrap().cents(), Amount::MIN);
    }

    #[test]
    fn should_create_amount_when_value_is_equal_max() {
        let amount = Amount::new(Amount::MAX);

        assert!(amount.is_ok());
        assert_eq!(amount.unwrap().cents(), Amount::MAX);
    }

    #[test]
    fn should_reject_amount_when_value_is_below_min() {
        let amount = Amount::new(Amount::MIN - 1);

        assert!(amount.is_err());
    }

    #[test]
    fn should_reject_amount_when_value_is_above_max() {
        let amount = Amount::new(Amount::MAX + 1);

        assert!(amount.is_err());
    }

    #[test]
    fn should_return_cents_when_amount_is_valid() {
        let amount = Amount::new(150).unwrap();

        assert_eq!(amount.cents(), 150);
    }

    #[test]
    fn should_format_currency_when_value_has_no_cents() {
        let amount = Amount::new(100).unwrap();

        assert_eq!(amount.currency(), "1.00");
    }

    #[test]
    fn should_format_currency_when_value_has_cents() {
        let amount = Amount::new(123).unwrap();

        assert_eq!(amount.currency(), "1.23");
    }

    #[test]
    fn should_format_currency_when_large_value_is_valid() {
        let amount = Amount::new(1_000_000).unwrap();

        assert_eq!(amount.currency(), "10000.00");
    }
}