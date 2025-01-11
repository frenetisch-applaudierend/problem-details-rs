pub(crate) mod uri {
    use std::fmt;

    use http::Uri;
    use serde::{
        de::{self, Unexpected},
        Serializer,
    };

    struct UriVisitor;

    impl serde::de::Visitor<'_> for UriVisitor {
        type Value = Option<Uri>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "valid uri")
        }

        fn visit_str<E: de::Error>(self, val: &str) -> Result<Self::Value, E> {
            let uri = val
                .parse()
                .map_err(|_| de::Error::invalid_value(Unexpected::Str(val), &self))?;
            Ok(Some(uri))
        }

        fn visit_string<E: de::Error>(self, val: String) -> Result<Self::Value, E> {
            let uri = val.try_into().map_err(de::Error::custom)?;
            Ok(Some(uri))
        }

        fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }
    }

    pub fn serialize<S: Serializer>(value: &http::Uri, serializer: S) -> Result<S::Ok, S::Error> {
        http_serde::uri::serialize(value, serializer)
    }

    pub fn deserialize<'de, D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<http::Uri, D::Error> {
        http_serde::uri::deserialize(deserializer)
    }

    pub mod opt {

        pub fn serialize<S: serde::Serializer>(
            value: &Option<http::Uri>,
            serializer: S,
        ) -> Result<S::Ok, S::Error> {
            match value {
                Some(ref value) => super::serialize(value, serializer),
                None => serializer.serialize_none(),
            }
        }

        pub fn deserialize<'de, D: serde::Deserializer<'de>>(
            deserializer: D,
        ) -> Result<Option<http::Uri>, D::Error> {
            deserializer.deserialize_str(super::UriVisitor)
        }
    }
}

pub(crate) mod status {
    use std::fmt;

    use http::StatusCode;
    use serde::de::{self, Unexpected, Visitor};

    struct StatusVisitor;

    impl Visitor<'_> for StatusVisitor {
        type Value = Option<StatusCode>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "valid status code")
        }

        fn visit_i64<E: de::Error>(self, val: i64) -> Result<Self::Value, E> {
            let val: u16 = val
                .try_into()
                .map_err(|_| de::Error::invalid_value(Unexpected::Signed(val), &self))?;

            self.visit_u16(val)
        }

        fn visit_u64<E: de::Error>(self, val: u64) -> Result<Self::Value, E> {
            let val: u16 = val
                .try_into()
                .map_err(|_| de::Error::invalid_value(Unexpected::Unsigned(val), &self))?;

            self.visit_u16(val)
        }

        fn visit_u16<E: de::Error>(self, val: u16) -> Result<Self::Value, E> {
            let status_code = StatusCode::from_u16(val)
                .map_err(|_| de::Error::invalid_value(Unexpected::Unsigned(val.into()), &self))?;

            Ok(Some(status_code))
        }

        fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }
    }

    pub fn serialize<S: serde::Serializer>(
        value: &http::StatusCode,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        http_serde::status_code::serialize(value, serializer)
    }

    // pub fn deserialize<'de, D: serde::Deserializer<'de>>(
    //     deserializer: D,
    // ) -> Result<http::StatusCode, D::Error> {
    //     http_serde::status_code::deserialize(deserializer)
    // }

    pub mod opt {
        use super::StatusVisitor;

        pub fn serialize<S: serde::Serializer>(
            value: &Option<http::StatusCode>,
            serializer: S,
        ) -> Result<S::Ok, S::Error> {
            match value {
                Some(ref value) => super::serialize(value, serializer),
                None => serializer.serialize_none(),
            }
        }

        pub fn deserialize<'de, D: serde::Deserializer<'de>>(
            deserializer: D,
        ) -> Result<Option<http::StatusCode>, D::Error> {
            deserializer.deserialize_u16(StatusVisitor)
        }
    }
}
