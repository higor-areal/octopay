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