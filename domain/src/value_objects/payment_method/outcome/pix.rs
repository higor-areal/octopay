use crate::value_objects::expiration::Expiration;


pub struct PixData {
    qr_code: String,
    qr_code_base64: Option<String>,
    qr_code_url: Option<String>,
    expires_at: Expiration,
}

impl PixData {
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