use crate::value_objects::error::ValidationError;


#[derive(Debug, Clone, PartialEq)]
pub struct Email(String);

#[allow(dead_code)]
impl Email {
    pub fn new(email: impl Into<String>) -> Result<Self, ValidationError> {
        let email = email.into().trim().to_lowercase();

        if email.is_empty() {
            return Err(ValidationError::InvalidEmail);
        }

        if email.len() > 254 {
            return Err(ValidationError::InvalidEmail);
        }

        let (local, domain) = email
            .split_once('@')
            .ok_or(ValidationError::InvalidEmail)?;

        if local.is_empty() {
            return Err(ValidationError::InvalidEmail);
        }

        if domain.is_empty() {
            return Err(ValidationError::InvalidEmail);
        }

        if !domain.contains('.') {
            return Err(ValidationError::InvalidEmail);
        }

        Ok(Self(email))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_email_when_valid() {
        let email = Email::new("john@example.com").unwrap();

        assert_eq!(email.as_str(), "john@example.com");
    }

    #[test]
    fn should_trim_spaces_when_creating_email() {
        let email = Email::new("  john@example.com  ").unwrap();

        assert_eq!(email.as_str(), "john@example.com");
    }

    #[test]
    fn should_convert_email_to_lowercase() {
        let email = Email::new("John.DOE@Example.COM").unwrap();

        assert_eq!(email.as_str(), "john.doe@example.com");
    }

    #[test]
    fn should_reject_empty_email() {
        let result = Email::new("");

        assert!(matches!(result, Err(ValidationError::InvalidEmail)));
    }

    #[test]
    fn should_reject_email_with_only_spaces() {
        let result = Email::new("     ");

        assert!(matches!(result, Err(ValidationError::InvalidEmail)));
    }

    #[test]
    fn should_reject_email_without_at_symbol() {
        let result = Email::new("john.example.com");

        assert!(matches!(result, Err(ValidationError::InvalidEmail)));
    }

    #[test]
    fn should_reject_email_without_local_part() {
        let result = Email::new("@example.com");

        assert!(matches!(result, Err(ValidationError::InvalidEmail)));
    }

    #[test]
    fn should_reject_email_without_domain() {
        let result = Email::new("john@");

        assert!(matches!(result, Err(ValidationError::InvalidEmail)));
    }

    #[test]
    fn should_reject_email_without_top_level_domain() {
        let result = Email::new("john@example");

        assert!(matches!(result, Err(ValidationError::InvalidEmail)));
    }

    #[test]
    fn should_accept_email_with_subdomain() {
        let email = Email::new("john@mail.example.com").unwrap();

        assert_eq!(email.as_str(), "john@mail.example.com");
    }

    #[test]
    fn should_accept_email_with_plus_alias() {
        let email = Email::new("john+payments@example.com").unwrap();

        assert_eq!(email.as_str(), "john+payments@example.com");
    }

    #[test]
    fn should_accept_email_with_numbers() {
        let email = Email::new("john123@example123.com").unwrap();

        assert_eq!(email.as_str(), "john123@example123.com");
    }

    #[test]
    fn should_reject_email_when_length_exceeds_limit() {
        let local = "a".repeat(249);
        let email = format!("{}@a.com", local);

        assert_eq!(email.len(), 255);

        let result = Email::new(email);

        assert!(matches!(result, Err(ValidationError::InvalidEmail)));
    }

    #[test]
    fn should_accept_email_when_length_is_exactly_limit() {
        let local = "a".repeat(248);
        let email = format!("{}@a.com", local);

        let email = Email::new(email).unwrap();

        assert_eq!(email.as_str().len(), 254);
    }
}