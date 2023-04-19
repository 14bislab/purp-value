use crate::Value;

/// A trait for converting types to `Value`.
pub trait ToValueTrait {
    /// Converts a type into a `Value`.
    fn to_value(&self) -> Value;
}

/// A trait for converting `Value` to types.
pub trait FromValueTrait {
    /// Converts a `Value` into a type.
    fn from_value(value: Value) -> Self;
}

/// A trait for converting types to JSON strings.
pub trait ToJsonTrait {
    /// Converts a type into a JSON string.
    fn to_json(&self) -> String;
}

/// A trait for converting types to YAML strings.
pub trait ToYamlTrait {
    /// Converts a type into a YAML string.
    fn to_yaml(&self) -> String;
}

/// A trait for converting types to XML strings.
pub trait ToXmlTrait {
    /// Converts a type into an XML string.
    fn to_xml(&self) -> String;
}
