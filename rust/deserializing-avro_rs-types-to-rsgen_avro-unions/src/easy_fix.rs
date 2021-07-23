//! Fixes the issue easily by annotating the unnamed Avro union as serde untagged.
//! The only issue is that long values are incorrectly assigned to the Int variant.
//! This could be ok, as the data is at least available and can be recast by the user.

/// Auto-generated type for unnamed Avro union variants.
#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
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
