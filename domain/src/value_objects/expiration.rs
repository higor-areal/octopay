use chrono::{DateTime, Utc};

use crate::value_objects::error::ValidationError;


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expiration(DateTime<Utc>);

impl Expiration {

    pub fn new(date: DateTime<Utc>) -> Self {
        Self(date)
    }

    pub fn from_rfc3339(
        date: impl AsRef<str>,
    ) -> Result<Self, ValidationError> {
        let date = DateTime::parse_from_rfc3339(date.as_ref())
            .map_err(|_| ValidationError::InvalidExpirationDate)?
            .with_timezone(&Utc);

        Ok(Self::new(date))
    }

    pub fn date_time(&self) -> &DateTime<Utc> {
        &self.0
    }

    pub fn into_inner(self) -> DateTime<Utc> {
        self.0
    }

    pub fn is_expired(&self, now: DateTime<Utc>) -> bool {
        now >= self.0
    }
}

impl From<DateTime<Utc>> for Expiration {
    fn from(value: DateTime<Utc>) -> Self {
        Self::new(value)
    }
}

impl std::convert::TryFrom<&str> for Expiration {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_rfc3339(value)
    }
}

impl std::convert::TryFrom<String> for Expiration {
    type Error = ValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_rfc3339(value)
    }
}





#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, Utc};

    mod constructor {
        use super::*;

        #[test]
        fn should_create_from_datetime() {
            let date = DateTime::parse_from_rfc3339("2030-01-01T12:00:00Z")
                .unwrap()
                .with_timezone(&Utc);

            let expiration = Expiration::new(date);

            assert_eq!(expiration.date_time(), &date);
        }

        #[test]
        fn should_create_from_valid_rfc3339() {
            let expiration =
                Expiration::from_rfc3339("2030-01-01T12:00:00Z").unwrap();

            let expected = DateTime::parse_from_rfc3339("2030-01-01T12:00:00Z")
                .unwrap()
                .with_timezone(&Utc);

            assert_eq!(expiration.date_time(), &expected);
        }

        #[test]
        fn should_reject_invalid_rfc3339() {
            let result = Expiration::from_rfc3339("01/01/2030");

            assert!(matches!(
                result,
                Err(ValidationError::InvalidExpirationDate)
            ));
        }

        #[test]
        fn should_create_using_try_from_str() {
            let expiration =
                Expiration::try_from("2030-01-01T12:00:00Z").unwrap();

            assert_eq!(
                expiration.date_time(),
                &DateTime::parse_from_rfc3339("2030-01-01T12:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc)
            );
        }

        #[test]
        fn should_create_using_try_from_string() {
            let value = String::from("2030-01-01T12:00:00Z");

            let expiration = Expiration::try_from(value).unwrap();

            assert_eq!(
                expiration.date_time(),
                &DateTime::parse_from_rfc3339("2030-01-01T12:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc)
            );
        }
    }

    mod expiration {
        use super::*;

        fn expiration() -> Expiration {
            Expiration::from_rfc3339("2030-01-01T12:00:00Z").unwrap()
        }

        #[test]
        fn should_not_be_expired_when_now_is_before_expiration() {
            let now = DateTime::parse_from_rfc3339("2030-01-01T11:59:59Z")
                .unwrap()
                .with_timezone(&Utc);

            assert!(!expiration().is_expired(now));
        }

        #[test]
        fn should_be_expired_when_now_is_equal_to_expiration() {
            let now = DateTime::parse_from_rfc3339("2030-01-01T12:00:00Z")
                .unwrap()
                .with_timezone(&Utc);

            assert!(expiration().is_expired(now));
        }

        #[test]
        fn should_be_expired_when_now_is_after_expiration() {
            let now = DateTime::parse_from_rfc3339("2030-01-01T12:00:01Z")
                .unwrap()
                .with_timezone(&Utc);

            assert!(expiration().is_expired(now));
        }

        #[test]
        fn should_return_inner_datetime() {
            let expiration = expiration();
            let datetime = expiration.clone().into_inner();

            assert_eq!(expiration.date_time(), &datetime);
        }
    }
}