use crate::value_objects::payer::{document::Document, email::Email, name::Name};

#[allow(dead_code)]
pub struct Payer {
    name: Name,
    email: Email,
    document: Document,
}

impl Payer {
    pub fn new(name: Name, email: Email, document: Document) -> Self {
        Self {
            name,
            email,
            document,
        }
    }

    pub fn name(&self) -> &Name {
        &self.name
    }
    pub fn email(&self) -> &Email {
        &self.email
    }
    pub fn document(&self) -> &Document {
        &self.document
    }
}
