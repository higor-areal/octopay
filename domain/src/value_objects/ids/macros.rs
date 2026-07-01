macro_rules! id_type {
    ($name:ident, $error:path) => {
        #[derive(
            Debug,
            Clone,
            Copy,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Hash,
        )]
        pub struct $name(::uuid::Uuid);

        impl $name {
            /// Gera um novo UUID v7.
            pub fn generate() -> Self {
                Self(::uuid::Uuid::now_v7())
            }

            /// Cria um ID a partir de uma string UUID.
            pub fn new(value: impl AsRef<str>) -> Result<Self, $error> {
                let uuid = ::uuid::Uuid::parse_str(value.as_ref())
                    .map_err(|_| <$error>::InvalidUuid)?;

                Ok(Self(uuid))
            }

            /// Retorna uma referência para o UUID interno.
            pub fn as_uuid(&self) -> &::uuid::Uuid {
                &self.0
            }

            /// Consome o wrapper e retorna o UUID.
            pub fn into_uuid(self) -> ::uuid::Uuid {
                self.0
            }
        }

        impl ::std::default::Default for $name {
            fn default() -> Self {
                Self::generate()
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(
                &self,
                f: &mut ::std::fmt::Formatter<'_>,
            ) -> ::std::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl ::std::str::FromStr for $name {
            type Err = $error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Self::new(s)
            }
        }

        impl From<::uuid::Uuid> for $name {
            fn from(value: ::uuid::Uuid) -> Self {
                Self(value)
            }
        }

        impl From<$name> for ::uuid::Uuid {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl AsRef<::uuid::Uuid> for $name {
            fn as_ref(&self) -> &::uuid::Uuid {
                &self.0
            }
        }
    };
}

pub(crate) use id_type;