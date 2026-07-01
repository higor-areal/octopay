use crate::error::ValidationError;

pub struct Name(String);

#[allow(dead_code)]
impl Name {
    pub fn new(name: impl Into<String>) -> Result<Self, ValidationError> {
        let name = name.into().trim().to_string();

        Self::validate(&name)?;

        Ok(Self(name))
    }

    fn validate(name: &str) -> Result<(), ValidationError> {
        const MIN_LENGTH: usize = 3;
        const MAX_LENGTH: usize = 120;

        if name.len() < MIN_LENGTH || name.len() > MAX_LENGTH {
            return Err(ValidationError::InvalidName);
        }

        if name.is_empty() {
            return Err(ValidationError::InvalidName);
        }

        if name.starts_with('-')
            || name.ends_with('-')
            || name.starts_with('\'')
            || name.ends_with('\'')
        {
            return Err(ValidationError::InvalidName);
        }

        let mut previous_separator = false;

        for c in name.chars() {
            let is_valid =
                c.is_alphabetic() || c.is_whitespace() || c == '-' || c == '\'';

            if !is_valid {
                return Err(ValidationError::InvalidName);
            }

            let is_separator = c.is_whitespace() || c == '-' || c == '\'';

            if is_separator {
                if previous_separator {
                    return Err(ValidationError::InvalidName);
                }
                previous_separator = true;
            } else {
                previous_separator = false;
            }
        }

        if !name.chars().any(char::is_alphabetic) {
            return Err(ValidationError::InvalidName);
        }

        Ok(())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_name_when_valid() {
        let name = Name::new("John Doe").unwrap();

        assert_eq!(name.as_str(), "John Doe");
    }

    #[test]
    fn should_trim_spaces_when_creating_name() {
        let name = Name::new("  John Doe  ").unwrap();

        assert_eq!(name.as_str(), "John Doe");
    }

    #[test]
    fn should_accept_name_with_minimum_length() {
        let name = Name::new("Ana").unwrap();

        assert_eq!(name.as_str(), "Ana");
    }

    #[test]
    fn should_accept_name_with_maximum_length() {
        let value = "a".repeat(120);

        let name = Name::new(value.clone()).unwrap();

        assert_eq!(name.as_str(), value);
    }

    #[test]
    fn should_accept_name_with_hyphen() {
        let name = Name::new("Anne-Marie").unwrap();

        assert_eq!(name.as_str(), "Anne-Marie");
    }

    #[test]
    fn should_accept_name_with_apostrophe() {
        let name = Name::new("O'Connor").unwrap();

        assert_eq!(name.as_str(), "O'Connor");
    }

    #[test]
    fn should_accept_name_with_unicode_letters() {
        let name = Name::new("José Ávila").unwrap();

        assert_eq!(name.as_str(), "José Ávila");
    }

    #[test]
    fn should_reject_empty_name() {
        let result = Name::new("");

        assert!(matches!(result, Err(ValidationError::InvalidName)));
    }

    #[test]
    fn should_reject_name_with_only_spaces() {
        let result = Name::new("      ");

        assert!(matches!(result, Err(ValidationError::InvalidName)));
    }

    #[test]
    fn should_reject_name_shorter_than_minimum_length() {
        let result = Name::new("Al");

        assert!(matches!(result, Err(ValidationError::InvalidName)));
    }

    #[test]
    fn should_reject_name_longer_than_maximum_length() {
        let value = "a".repeat(121);

        let result = Name::new(value);

        assert!(matches!(result, Err(ValidationError::InvalidName)));
    }

    #[test]
    fn should_reject_name_starting_with_hyphen() {
        let result = Name::new("-John");

        assert!(matches!(result, Err(ValidationError::InvalidName)));
    }

    #[test]
    fn should_reject_name_ending_with_hyphen() {
        let result = Name::new("John-");

        assert!(matches!(result, Err(ValidationError::InvalidName)));
    }

    #[test]
    fn should_reject_name_starting_with_apostrophe() {
        let result = Name::new("'John");

        assert!(matches!(result, Err(ValidationError::InvalidName)));
    }

    #[test]
    fn should_reject_name_ending_with_apostrophe() {
        let result = Name::new("John'");

        assert!(matches!(result, Err(ValidationError::InvalidName)));
    }

    #[test]
    fn should_reject_name_with_double_spaces() {
        let result = Name::new("John  Doe");

        assert!(matches!(result, Err(ValidationError::InvalidName)));
    }

    #[test]
    fn should_reject_name_with_double_hyphen() {
        let result = Name::new("John--Doe");

        assert!(matches!(result, Err(ValidationError::InvalidName)));
    }

    #[test]
    fn should_reject_name_with_double_apostrophe() {
        let result = Name::new("O''Connor");

        assert!(matches!(result, Err(ValidationError::InvalidName)));
    }

    #[test]
    fn should_reject_name_with_space_before_hyphen() {
        let result = Name::new("John -Doe");

        assert!(matches!(result, Err(ValidationError::InvalidName)));
    }

    #[test]
    fn should_reject_name_with_space_after_hyphen() {
        let result = Name::new("John- Doe");

        assert!(matches!(result, Err(ValidationError::InvalidName)));
    }

    #[test]
    fn should_reject_name_with_space_before_apostrophe() {
        let result = Name::new("John 'Doe");

        assert!(matches!(result, Err(ValidationError::InvalidName)));
    }

    #[test]
    fn should_reject_name_with_space_after_apostrophe() {
        let result = Name::new("John' Doe");

        assert!(matches!(result, Err(ValidationError::InvalidName)));
    }

    #[test]
    fn should_reject_name_with_numbers() {
        let result = Name::new("John123");

        assert!(matches!(result, Err(ValidationError::InvalidName)));
    }

    #[test]
    fn should_reject_name_with_symbols() {
        let result = Name::new("John@Doe");

        assert!(matches!(result, Err(ValidationError::InvalidName)));
    }

    #[test]
    fn should_reject_name_without_letters() {
        let result = Name::new("---");

        assert!(matches!(result, Err(ValidationError::InvalidName)));
    }
}
