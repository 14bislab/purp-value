use crate::{to::json::JsonMode, value::TypeToValue, Array, Number, Object, StringB, Value};
use std::{
    collections::{BTreeMap, HashMap},
    fmt::{Display, Formatter},
};

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

impl Default for Value {
    fn default() -> Self {
        Value::Null
    }
}

impl From<BTreeMap<String, Value>> for Value {
    fn from(value: BTreeMap<String, Value>) -> Self {
        Value::Object(Object::BTreeMap(value))
    }
}

impl From<HashMap<String, Value>> for Value {
    fn from(value: HashMap<String, Value>) -> Self {
        Value::Object(Object::HashMap(value))
    }
}

impl From<Vec<Value>> for Value {
    fn from(value: Vec<Value>) -> Self {
        Array::from(value).to_value()
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        StringB::from(value).to_value()
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        StringB::from(value).to_value()
    }
}

impl From<StringB> for Value {
    fn from(value: StringB) -> Self {
        value.to_value()
    }
}

impl From<i8> for Value {
    fn from(value: i8) -> Self {
        Value::Number(Number {
            i8: Some(value),
            ..Default::default()
        })
    }
}

impl From<i16> for Value {
    fn from(value: i16) -> Self {
        Value::Number(Number {
            i16: Some(value),
            ..Default::default()
        })
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value::Number(Number {
            i32: Some(value),
            ..Default::default()
        })
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::Number(Number {
            i64: Some(value),
            ..Default::default()
        })
    }
}

impl From<i128> for Value {
    fn from(value: i128) -> Self {
        Value::Number(Number {
            i128: Some(value),
            ..Default::default()
        })
    }
}

impl From<u8> for Value {
    fn from(value: u8) -> Self {
        Value::Number(Number {
            u8: Some(value),
            ..Default::default()
        })
    }
}

impl From<u16> for Value {
    fn from(value: u16) -> Self {
        Value::Number(Number {
            u16: Some(value),
            ..Default::default()
        })
    }
}

impl From<u32> for Value {
    fn from(value: u32) -> Self {
        Value::Number(Number {
            u32: Some(value),
            ..Default::default()
        })
    }
}

impl From<u64> for Value {
    fn from(value: u64) -> Self {
        Value::Number(Number {
            u64: Some(value),
            ..Default::default()
        })
    }
}

impl From<u128> for Value {
    fn from(value: u128) -> Self {
        Value::Number(Number {
            u128: Some(value),
            ..Default::default()
        })
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Value::Number(Number {
            f32: Some(value),
            ..Default::default()
        })
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Number(Number {
            f64: Some(value),
            ..Default::default()
        })
    }
}

impl From<Number> for Value {
    fn from(value: Number) -> Self {
        Value::Number(value)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Boolean(value)
    }
}

impl From<()> for Value {
    fn from(_: ()) -> Self {
        Value::Null
    }
}

impl From<Option<BTreeMap<String, Value>>> for Value {
    fn from(value: Option<BTreeMap<String, Value>>) -> Self {
        match value {
            Some(value) => Object::from(value).to_value(),
            None => Value::Null,
        }
    }
}

impl From<Option<HashMap<String, Value>>> for Value {
    fn from(value: Option<HashMap<String, Value>>) -> Self {
        match value {
            Some(value) => Object::from(value).to_value(),
            None => Value::Null,
        }
    }
}

impl From<Option<Vec<Value>>> for Value {
    fn from(value: Option<Vec<Value>>) -> Self {
        match value {
            Some(value) => Array::from(value).to_value(),
            None => Value::Null,
        }
    }
}

impl From<Option<String>> for Value {
    fn from(value: Option<String>) -> Self {
        match value {
            Some(value) => StringB::from(value).to_value(),
            None => Value::Null,
        }
    }
}

impl<T> From<Option<T>> for Value
where
    T: Into<Number>,
{
    fn from(value: Option<T>) -> Self {
        match value {
            Some(value) => Value::Number(value.into().into()),
            None => Value::Null,
        }
    }
}
