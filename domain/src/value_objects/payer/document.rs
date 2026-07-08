use crate::value_objects::error::ValidationError;



#[derive(Debug, PartialEq)]
pub enum DocumentType {
    Cpf,
    Cnpj,
} 

impl DocumentType {
    pub fn new(value: &str) -> Result<Self, ValidationError> {
        match value.trim().to_ascii_lowercase().as_str() {
            "cpf" => Ok(Self::Cpf),
            "cnpj" => Ok(Self::Cnpj),
            _ => Err(ValidationError::InvalidDocument),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub struct Document {
    number: String,
    document_type: DocumentType,
}


#[allow(dead_code)]
impl Document {
    pub fn number(&self) -> &str {
        &self.number
    }

    pub fn document_type(&self) -> &DocumentType {
        &self.document_type
    }
}

#[allow(dead_code)]
impl Document {
    pub fn new(
        number: impl Into<String>,
        document_type: DocumentType,
    ) -> Result<Self, ValidationError> {
        let number = number.into();

        match document_type {
            DocumentType::Cpf => validar_cpf(&number)?,
            DocumentType::Cnpj => validar_cnpj(&number)?,
        }

        Ok(Self {
            number: Self::normalize(&number),
            document_type,
        })
    }

    fn normalize(document: &str) -> String {
        document
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect()
    }
}


pub fn validar_cpf(cpf: &str) -> Result<(), ValidationError> {
    let cpf = somente_numeros(cpf);

    if cpf.len() != 11 {
        return Err(ValidationError::InvalidDocument);
    }

    if todos_iguais(&cpf) {
        return Err(ValidationError::InvalidDocument);
    }

    let digits = str_para_digits(&cpf)?;

    let dv1 = calcular_digito(&digits[..9], 10);
    let dv2 = calcular_digito(&digits[..10], 11);

    if digits[9] != dv1 || digits[10] != dv2 {
        return Err(ValidationError::InvalidDocument);
    }

    Ok(())
}

pub fn validar_cnpj(cnpj: &str) -> Result<(), ValidationError> {
    let cnpj = somente_numeros(cnpj);

    if cnpj.len() != 14 {
        return Err(ValidationError::InvalidDocument);
    }

    if todos_iguais(&cnpj) {
        return Err(ValidationError::InvalidDocument);
    }

    let digits = str_para_digits(&cnpj)?;

    let pesos_dv1 = [5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
    let pesos_dv2 = [6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];

    let soma1: u32 = digits[..12]
        .iter()
        .zip(pesos_dv1.iter())
        .map(|(d, p)| d * p)
        .sum();

    let dv1 = match soma1 % 11 {
        0 | 1 => 0,
        resto => 11 - resto,
    };

    let soma2: u32 = digits[..13]
        .iter()
        .zip(pesos_dv2.iter())
        .map(|(d, p)| d * p)
        .sum();

    let dv2 = match soma2 % 11 {
        0 | 1 => 0,
        resto => 11 - resto,
    };

    if digits[12] != dv1 || digits[13] != dv2 {
        return Err(ValidationError::InvalidDocument);
    }

    Ok(())
}

fn somente_numeros(input: &str) -> String {
    input.chars()
        .filter(|c| c.is_ascii_digit())
        .collect()
}

fn todos_iguais(input: &str) -> bool {
    input
        .chars()
        .all(|c| c == input.chars().next().unwrap())
}

fn str_para_digits(input: &str) -> Result<Vec<u32>, ValidationError> {
    input
        .chars()
        .map(|c| {
            c.to_digit(10)
                .ok_or(ValidationError::InvalidDocument)
        })
        .collect()
}

fn calcular_digito(base: &[u32], peso_inicial: u32) -> u32 {
    let soma: u32 = base
        .iter()
        .zip((2..=peso_inicial).rev())
        .map(|(d, p)| d * p)
        .sum();

    let resto = soma % 11;

    if resto < 2 {
        0
    } else {
        11 - resto
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    mod document_type {
        use super::*;

        #[test]
        fn should_create_cpf_document_type() {
            let result = DocumentType::new("cpf");

            assert_eq!(result.unwrap(), DocumentType::Cpf);
        }

        #[test]
        fn should_create_cnpj_document_type() {
            let result = DocumentType::new("cnpj");

            assert_eq!(result.unwrap(), DocumentType::Cnpj);
        }

        #[test]
        fn should_ignore_case_when_creating_document_type() {
            let result = DocumentType::new("CPF");

            assert_eq!(result.unwrap(), DocumentType::Cpf);
        }

        #[test]
        fn should_trim_spaces_when_creating_document_type() {
            let result = DocumentType::new("  cnpj  ");

            assert_eq!(result.unwrap(), DocumentType::Cnpj);
        }

        #[test]
        fn should_reject_invalid_document_type() {
            let result = DocumentType::new("passport");

            assert!(matches!(result, Err(ValidationError::InvalidDocument)));
        }
    }

    mod cpf {
        use super::*;

        #[test]
        fn should_accept_valid_cpf() {
            assert!(validar_cpf("52998224725").is_ok());
        }

        #[test]
        fn should_accept_formatted_cpf() {
            assert!(validar_cpf("529.982.247-25").is_ok());
        }

        #[test]
        fn should_reject_invalid_check_digits() {
            assert!(matches!(
                validar_cpf("52998224724"),
                Err(ValidationError::InvalidDocument)
            ));
        }

        #[test]
        fn should_reject_invalid_length() {
            assert!(matches!(
                validar_cpf("123456789"),
                Err(ValidationError::InvalidDocument)
            ));
        }

        #[test]
        fn should_reject_all_equal_digits() {
            assert!(matches!(
                validar_cpf("11111111111"),
                Err(ValidationError::InvalidDocument)
            ));
        }

        #[test]
        fn should_reject_letters() {
            assert!(matches!(
                validar_cpf("52998224A25"),
                Err(ValidationError::InvalidDocument)
            ));
        }
    }

    mod cnpj {
        use super::*;

        #[test]
        fn should_accept_valid_cnpj() {
            assert!(validar_cnpj("11444777000161").is_ok());
        }

        #[test]
        fn should_accept_formatted_cnpj() {
            assert!(validar_cnpj("11.444.777/0001-61").is_ok());
        }

        #[test]
        fn should_reject_invalid_check_digits() {
            assert!(matches!(
                validar_cnpj("11444777000160"),
                Err(ValidationError::InvalidDocument)
            ));
        }

        #[test]
        fn should_reject_invalid_length() {
            assert!(matches!(
                validar_cnpj("123"),
                Err(ValidationError::InvalidDocument)
            ));
        }

        #[test]
        fn should_reject_all_equal_digits() {
            assert!(matches!(
                validar_cnpj("11111111111111"),
                Err(ValidationError::InvalidDocument)
            ));
        }

        #[test]
        fn should_reject_letters() {
            assert!(matches!(
                validar_cnpj("1144477700016A"),
                Err(ValidationError::InvalidDocument)
            ));
        }
    }

    mod document {
        use super::*;

        #[test]
        fn should_create_document_with_valid_cpf() {
            let document =
                Document::new("529.982.247-25", DocumentType::Cpf).unwrap();

            assert_eq!(document.number(), "52998224725");
            assert_eq!(document.document_type(), &DocumentType::Cpf);
        }

        #[test]
        fn should_create_document_with_valid_cnpj() {
            let document =
                Document::new("11.444.777/0001-61", DocumentType::Cnpj).unwrap();

            assert_eq!(document.number(), "11444777000161");
            assert_eq!(document.document_type(), &DocumentType::Cnpj);
        }

        #[test]
        fn should_normalize_document_number() {
            let document =
                Document::new("529.982.247-25", DocumentType::Cpf).unwrap();

            assert_eq!(document.number(), "52998224725");
        }

        #[test]
        fn should_reject_invalid_cpf() {
            let result =
                Document::new("11111111111", DocumentType::Cpf);

            assert!(matches!(result, Err(ValidationError::InvalidDocument)));
        }

        #[test]
        fn should_reject_invalid_cnpj() {
            let result =
                Document::new("11111111111111", DocumentType::Cnpj);

            assert!(matches!(result, Err(ValidationError::InvalidDocument)));
        }
    }
}