//! This module provides a `Value` enum to represent different data types and
//! a trait `TypeToValue` to convert them to `Value`. The supported data types
//! are: String, Number, Boolean, Array, Object, Null, Undefined, and DateTime.
//!
//! # Examples
//!
//! ```
//! use crate::{Array, DateTime, Number, Object, StringB, Value};
//!
//! let string_value = Value::String(StringB::new("hello".to_string()));
//! let number_value = Value::Number(Number::from(42));
//! let boolean_value = Value::Boolean(true);
//! let null_value = Value::Null;
//! let undefined_value = Value::Undefined;
//! let mut datetime_value = Value::DateTime(DateTime::from("2023-04-05T00:00:00Z"));
//! ```
use crate::{Array, DateTime, Number, Object, StringB};

/// A trait to convert a data type to a `Value` enum.
pub trait TypeToValue {
    /// Converts the data type to a `Value` enum.
    fn to_value(&self) -> Value;
}

/// Represents different data types as an enum.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(StringB),
    Number(Number),
    Boolean(bool),
    Array(Array),
    Object(Object),
    Null,
    Undefined,
    DateTime(DateTime),
}

#[cfg(test)]
mod tests {
    use super::{Array, DateTime, Number, Object, StringB, Value};

    // Tests for the different data types and their conversion to a `Value` enum.
    #[test]
    fn test_value_string() {
        let string = StringB::new("hello".to_string());
        let value = Value::String(string.clone());
        assert_eq!(value, Value::String(string));
    }

    #[test]
    fn test_value_number() {
        let number = Number::from(42);
        let value = Value::Number(number);
        assert_eq!(value, Value::Number(Number::from(42)));
    }

    #[test]
    fn test_value_boolean() {
        let value = Value::Boolean(true);
        assert_eq!(value, Value::Boolean(true));
    }

    #[test]
    fn test_value_array() {
        let mut array = Array::new();
        array.push(Value::Number(Number::from(1)));
        array.push(Value::Number(Number::from(2)));
        let value = Value::Array(array.clone());
        assert_eq!(value, Value::Array(array));
    }

    #[test]
    fn test_value_object() {
        let mut object = Object::default();
        object.insert(
            "key".to_string(),
            Value::String(StringB::new("value".to_string())),
        );
        let value = Value::Object(object.clone());
        assert_eq!(value, Value::Object(object));
    }

    #[test]
    fn test_value_null() {
        let value = Value::Null;
        assert_eq!(value, Value::Null);
    }

    #[test]
    fn test_value_undefined() {
        let value = Value::Undefined;
        assert_eq!(value, Value::Undefined);
    }

    #[test]
    fn test_value_datetime() {
        let datetime = DateTime::from("2023-04-05T00:00:00Z");
        let value = Value::DateTime(datetime.clone());
        assert_eq!(value, Value::DateTime(datetime));
    }
}
