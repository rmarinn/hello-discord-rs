use serde::{de, Deserialize};

/// Represents a value that can be one of multiple types.
///
/// This enum allows for the representation of different types of values, including:
/// - `String`: A string value.
/// - `Integer`: A signed integer value.
/// - `Double`: A floating-point number.
/// - `Boolean`: A boolean value (true or false).
#[derive(Debug)]
pub enum MultiTypeValue {
    /// A string value.
    String(String),
    /// An integer value.
    Integer(i64),
    /// A floating-point number.
    Double(f64),
    /// A boolean value (true or false).
    Boolean(bool),
}

impl<'de> Deserialize<'de> for MultiTypeValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct MultiTypeValueVisitor;

        impl<'de> de::Visitor<'de> for MultiTypeValueVisitor {
            type Value = MultiTypeValue;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string, integer, double, or boolean value")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(MultiTypeValue::String(value.to_owned()))
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(MultiTypeValue::Integer(value))
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                // Cast u64 to i64 (if safe), or handle as double
                if value <= i64::MAX as u64 {
                    Ok(MultiTypeValue::Integer(value as i64))
                } else {
                    Ok(MultiTypeValue::Double(value as f64))
                }
            }

            fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(MultiTypeValue::Double(value))
            }

            fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(MultiTypeValue::Boolean(value))
            }
        }

        deserializer.deserialize_any(MultiTypeValueVisitor)
    }
}
