use chrono::{DateTime, Utc};
use crate::error::ValidationError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expiration(DateTime<Utc>);

impl Expiration {
    pub fn new(date: impl AsRef<str>) -> Result<Self, ValidationError> {
        let date = DateTime::parse_from_rfc3339(date.as_ref())
            .map_err(|_| ValidationError::InvalidExpirationDate)?
            .with_timezone(&Utc);

        Ok(Self(date))
    }

    pub fn date_time(&self) -> &DateTime<Utc> {
        &self.0
    }

    pub fn is_expired(&self, now: &DateTime<Utc>) -> bool {
        *now >= self.0
    }
}