use crate::value_objects::expiration::Expiration;

pub struct PixData {
    qr_code: String,
    qr_code_base64: Option<String>,
    qr_code_url: Option<String>,
    expires_at: Expiration,
}

impl PixData {
    pub fn new(
        qr_code: impl Into<String>,
        qr_code_base64: Option<impl Into<String>>,
        qr_code_url: Option<impl Into<String>>,
        expires_at: Expiration,
    ) -> Self {
        Self {
            qr_code: qr_code.into(),
            qr_code_base64: qr_code_base64.map(Into::into),
            qr_code_url: qr_code_url.map(Into::into),
            expires_at,
        }
    }

    pub fn qr_code(&self) -> &str {
        &self.qr_code
    }

    pub fn qr_code_base64(&self) -> Option<&str> {
        self.qr_code_base64.as_deref()
    }

    pub fn qr_code_url(&self) -> Option<&str> {
        self.qr_code_url.as_deref()
    }

    pub fn expires_at(&self) -> &Expiration {
        &self.expires_at
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn expiration() -> Expiration {
        Expiration::from_rfc3339("2030-01-01T12:00:00Z").unwrap()
    }

    fn pix_data_valid() -> PixData {
        PixData::new(
            "qr_code",
            Some("qr_code_base64"),
            Some("https://example.com/qr"),
            expiration(),
        )
    }

    mod constructor {
        use super::*;

        #[test]
        fn should_create_pix_data() {
            let pix = pix_data_valid();

            assert_eq!(pix.qr_code(), "qr_code");
            assert_eq!(pix.qr_code_base64(), Some("qr_code_base64"));
            assert_eq!(pix.qr_code_url(), Some("https://example.com/qr"));
        }

        #[test]
        fn should_create_pix_data_without_base64() {
            let pix = PixData::new(
                "qr_code",
                None::<String>,
                Some("https://example.com/qr"),
                expiration(),
            );

            assert_eq!(pix.qr_code(), "qr_code");
            assert_eq!(pix.qr_code_base64(), None);
            assert_eq!(pix.qr_code_url(), Some("https://example.com/qr"));
        }

        #[test]
        fn should_create_pix_data_without_url() {
            let pix = PixData::new(
                "qr_code",
                Some("qr_code_base64"),
                None::<String>,
                expiration(),
            );

            assert_eq!(pix.qr_code(), "qr_code");
            assert_eq!(pix.qr_code_base64(), Some("qr_code_base64"));
            assert_eq!(pix.qr_code_url(), None);
        }

        #[test]
        fn should_create_pix_data_without_optional_fields() {
            let pix = PixData::new(
                "qr_code",
                None::<String>,
                None::<String>,
                expiration(),
            );

            assert_eq!(pix.qr_code(), "qr_code");
            assert_eq!(pix.qr_code_base64(), None);
            assert_eq!(pix.qr_code_url(), None);
        }

        #[test]
        fn should_return_expiration() {
            let expiration = expiration();

            let pix = PixData::new(
                "qr_code",
                None::<String>,
                None::<String>,
                expiration.clone(),
            );

            assert_eq!(pix.expires_at(), &expiration);
        }

        #[test]
        fn should_preserve_qr_code_value() {
            let qr = "00020101021226850014BR.GOV.BCB.PIX";

            let pix = PixData::new(
                qr,
                None::<String>,
                None::<String>,
                expiration(),
            );

            assert_eq!(pix.qr_code(), qr);
        }
    }
}


