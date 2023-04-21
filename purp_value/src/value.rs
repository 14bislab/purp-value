//! This module provides a `Value` enum to represent different data types and
//! a trait `ToValueBehavior` to convert them to `Value`. The supported data types
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
use crate::prelude::*;

use std::collections::{BTreeMap, HashMap};
use std::fmt::{Display, Formatter};
use std::ops::Deref;

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

impl Default for Value {
    fn default() -> Self {
        Value::Null
    }
}

impl ValueTrait for Value {}

impl ToValueBehavior for u8 {
    fn to_value(&self) -> Value {
        Value::Number(Number::from(*self))
    }
}

impl ToValueBehavior for u16 {
    fn to_value(&self) -> Value {
        Value::Number(Number::from(*self))
    }
}

impl ToValueBehavior for u32 {
    fn to_value(&self) -> Value {
        Value::Number(Number::from(*self))
    }
}

impl ToValueBehavior for u64 {
    fn to_value(&self) -> Value {
        Value::Number(Number::from(*self))
    }
}

impl ToValueBehavior for i8 {
    fn to_value(&self) -> Value {
        Value::Number(Number::from(*self))
    }
}

impl ToValueBehavior for i16 {
    fn to_value(&self) -> Value {
        Value::Number(Number::from(*self))
    }
}

impl ToValueBehavior for i32 {
    fn to_value(&self) -> Value {
        Value::Number(Number::from(*self))
    }
}

impl ToValueBehavior for i64 {
    fn to_value(&self) -> Value {
        Value::Number(Number::from(*self))
    }
}

impl ToValueBehavior for f32 {
    fn to_value(&self) -> Value {
        Value::Number(Number::from(*self))
    }
}

impl ToValueBehavior for f64 {
    fn to_value(&self) -> Value {
        Value::Number(Number::from(*self))
    }
}

impl ToValueBehavior for String {
    fn to_value(&self) -> Value {
        Value::String(StringB::new(self.clone()))
    }
}

impl ToValueBehavior for &str {
    fn to_value(&self) -> Value {
        Value::String(StringB::new(self.to_string()))
    }
}

impl ToValueBehavior for bool {
    fn to_value(&self) -> Value {
        Value::Boolean(*self)
    }
}

impl<T> ToValueBehavior for BTreeMap<String, T>
where
    T: ToValueBehavior,
{
    fn to_value(&self) -> Value {
        let mut object = Object::default();
        for (key, value) in self {
            object.insert(key.to_string(), value.to_value());
        }
        Value::Object(object)
    }
}

impl<T> ToValueBehavior for HashMap<String, T>
where
    T: ToValueBehavior,
{
    fn to_value(&self) -> Value {
        let mut object = Object::default();
        for (key, value) in self {
            object.insert(key.to_string(), value.to_value());
        }
        Value::Object(object)
    }
}

impl<T> ToValueBehavior for Vec<T>
where
    T: ToValueBehavior,
{
    fn to_value(&self) -> Value {
        let mut array = Array::new();
        for value in self {
            array.push(value.to_value());
        }
        Value::Array(array)
    }
}

impl<T> ToValueBehavior for Option<T>
where
    T: ToValueBehavior,
{
    fn to_value(&self) -> Value {
        match self {
            Some(value) => value.to_value(),
            None => Value::Null,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(_) => write!(f, "{}", self.to_json(JsonMode::Indented)),
            Value::Number(value) => write!(f, "{}", value),
            Value::Boolean(value) => write!(f, "{}", if *value { "true" } else { "false" }),
            Value::Array(_) => write!(f, "{}", self.to_json(JsonMode::Indented)),
            Value::Object(_) => write!(f, "{}", self.to_json(JsonMode::Indented)),
            Value::Null => write!(f, "null"),
            Value::Undefined => write!(f, "undefined"),
            Value::DateTime(value) => write!(f, "{}", value),
        }
    }
}

impl From<()> for Value {
    fn from(_: ()) -> Self {
        Value::Null
    }
}

impl<T> From<T> for Value
where
    T: ToValueBehavior,
{
    fn from(value: T) -> Self {
        value.to_value()
    }
}

impl<T> ToValueBehavior for HashMap<T, Value>
where
    T: ValueKeyBehavior,
{
    fn to_value(&self) -> Value {
        Object::from(self.clone()).to_value()
    }
}

impl ToValueBehavior for BTreeMap<String, Value> {
    fn to_value(&self) -> Value {
        Object::from(self.clone()).to_value()
    }
}

impl ToValueBehavior for Vec<Value> {
    fn to_value(&self) -> Value {
        Array::from(self.clone()).to_value()
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

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
