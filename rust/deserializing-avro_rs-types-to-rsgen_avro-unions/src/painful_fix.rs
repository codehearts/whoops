//! Fixes the issue painfully by implementing a serde visitor and deserializer.
//! The overhead is large, but types are deserialized to their correct variants.
//! Note that the auto-generated type can not derive `serde::Deserialize`.

use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer};
use std::fmt;

/// Auto-generated type for unnamed Avro union variants.
#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub enum UnionIntLongBool {
    Int(i32),
    Long(i64),
    Bool(bool),
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Event {
    pub a: Option<UnionIntLongBool>,
}

impl Default for Event {
    fn default() -> Event {
        Event { a: None }
    }
}

/// Visitor for the auto-generated unnamed Avro union type.
struct UnionIntLongBoolVisitor;

impl<'de> Visitor<'de> for UnionIntLongBoolVisitor {
    type Value = UnionIntLongBool;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a UnionIntLongBool")
    }

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(UnionIntLongBool::Int(value))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(UnionIntLongBool::Long(value))
    }

    fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(UnionIntLongBool::Bool(value))
    }
}

impl<'de> Deserialize<'de> for UnionIntLongBool {
    fn deserialize<D>(deserializer: D) -> Result<UnionIntLongBool, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(UnionIntLongBoolVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use avro_rs::{from_value, types::Value};

    #[test]
    fn it_works_with_null() -> Result<(), Box<dyn std::error::Error>> {
        let record = Value::Record(vec![("a".to_string(), Value::Union(Box::new(Value::Null)))]);

        let actual = from_value::<Event>(&record)?;
        let expected = Event { a: None };

        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn it_works_with_int() -> Result<(), Box<dyn std::error::Error>> {
        let record = Value::Record(vec![(
            "a".to_string(),
            Value::Union(Box::new(Value::Int(123))),
        )]);

        let actual = from_value::<Event>(&record)?;
        let expected = Event {
            a: Some(UnionIntLongBool::Int(123)),
        };

        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn it_works_with_long() -> Result<(), Box<dyn std::error::Error>> {
        let record = Value::Record(vec![(
            "a".to_string(),
            Value::Union(Box::new(Value::Long(123))),
        )]);

        let actual = from_value::<Event>(&record)?;
        let expected = Event {
            a: Some(UnionIntLongBool::Long(123)),
        };

        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn it_works_with_bool() -> Result<(), Box<dyn std::error::Error>> {
        let record = Value::Record(vec![(
            "a".to_string(),
            Value::Union(Box::new(Value::Boolean(true))),
        )]);

        let actual = from_value::<Event>(&record)?;
        let expected = Event {
            a: Some(UnionIntLongBool::Bool(true)),
        };

        assert_eq!(expected, actual);
        Ok(())
    }
}
