use crate::value_objects::expiration::Expiration;

pub struct BoletoData {
    barcode: String,
    digitable_line: String,
    pdf_url: Option<String>,
    expires_at: Expiration,
}

impl BoletoData {
    pub fn new(
        barcode: impl Into<String>,
        digitable_line: impl Into<String>,
        pdf_url: Option<String>,
        expires_at: Expiration,
    ) -> Self {
        Self {
            barcode: barcode.into(),
            digitable_line: digitable_line.into(),
            pdf_url,
            expires_at,
        }
    }

    pub fn barcode(&self) -> &str {
        &self.barcode
    }

    pub fn digitable_line(&self) -> &str {
        &self.digitable_line
    }

    pub fn pdf_url(&self) -> Option<&str> {
        self.pdf_url.as_deref()
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

    fn boleto_data_valid() -> BoletoData {
        BoletoData::new(
            "34191790010104351004791020150008291070026000",
            "34191.79001 01043.510047 91020.150008 2 91070026000",
            Some("https://example.com/boleto.pdf".to_string()),
            expiration(),
        )
    }

    mod constructor {
        use super::*;

        #[test]
        fn should_create_boleto_data() {
            let boleto = boleto_data_valid();

            assert_eq!(
                boleto.barcode(),
                "34191790010104351004791020150008291070026000"
            );

            assert_eq!(
                boleto.digitable_line(),
                "34191.79001 01043.510047 91020.150008 2 91070026000"
            );

            assert_eq!(
                boleto.pdf_url(),
                Some("https://example.com/boleto.pdf")
            );
        }

        #[test]
        fn should_create_boleto_data_without_pdf_url() {
            let boleto = BoletoData::new(
                "123456",
                "123.456",
                None,
                expiration(),
            );

            assert_eq!(boleto.barcode(), "123456");
            assert_eq!(boleto.digitable_line(), "123.456");
            assert_eq!(boleto.pdf_url(), None);
        }

        #[test]
        fn should_return_expiration() {
            let expiration = expiration();

            let boleto = BoletoData::new(
                "123456",
                "123.456",
                None,
                expiration.clone(),
            );

            assert_eq!(boleto.expires_at(), &expiration);
        }

        #[test]
        fn should_preserve_barcode() {
            let barcode = "12345678901234567890";

            let boleto = BoletoData::new(
                barcode,
                "linha",
                None,
                expiration(),
            );

            assert_eq!(boleto.barcode(), barcode);
        }

        #[test]
        fn should_preserve_digitable_line() {
            let line = "00190.00009 01234.567890 12345.678901 1 12340000010000";

            let boleto = BoletoData::new(
                "123456",
                line,
                None,
                expiration(),
            );

            assert_eq!(boleto.digitable_line(), line);
        }

        #[test]
        fn should_return_pdf_url_when_present() {
            let boleto = boleto_data_valid();

            assert_eq!(
                boleto.pdf_url(),
                Some("https://example.com/boleto.pdf")
            );
        }
    }
}