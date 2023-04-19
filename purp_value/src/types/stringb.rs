//! A custom string implementation called `StringB` that provides additional string manipulation methods.
//!
//! This implementation offers a way to handle strings with additional features, such as converting
//! the string to uppercase or lowercase, trimming, replacing, and concatenating. It also handles
//! converting between different representations of strings, such as `CString`, `String`, and `Vec<u8>`.
#[cfg(feature = "cstring")]
use std::ffi::CString;
use std::fmt::{Display, Formatter};
use std::ops::Deref;

use crate::value::TypeToValue;

/// A custom string implementation with additional manipulation methods.
#[derive(Debug, Clone, PartialEq)]
pub struct StringB {
    #[cfg(feature = "cstring")]
    pub value: CString,
    #[cfg(not(feature = "cstring"))]
    pub value: String,
}

impl TypeToValue for StringB {
    fn to_value(&self) -> crate::value::Value {
        crate::value::Value::String(StringB::from(self.clone()))
    }
}

impl StringB {
    #[cfg(feature = "cstring")]
    pub fn new<S: Into<CString>>(value: S) -> Self {
        StringB {
            value: value.into(),
        }
    }

    /// Creates a new instance of `StringB` with the provided value.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let s = StringB::new("hello");
    /// ```
    pub fn new<S: Into<String>>(value: S) -> Self {
        StringB {
            value: value.into(),
        }
    }

    #[cfg(feature = "cstring")]
    pub fn as_bytes(&self) -> &[u8] {
        self.value.to_bytes()
    }

    /// Gets the byte representation of the string.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let s = StringB::new("hello");
    /// let bytes = s.as_bytes();
    /// ```
    pub fn as_bytes(&self) -> &[u8] {
        self.value.as_bytes()
    }

    #[cfg(feature = "cstring")]
    pub fn as_str(&self) -> &str {
        self.value.to_str().expect("CString is not valid UTF-8")
    }

    /// Gets the string slice representation of the value.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let s = StringB::new("hello");
    /// let slice = s.as_str();
    /// ```
    pub fn as_str(&self) -> &str {
        self.value.as_str()
    }

    #[cfg(feature = "cstring")]
    pub fn to_string(&self) -> String {
        self.as_str().to_string()
    }

    /// Converts the value to a `String`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let s = StringB::new("hello");
    /// let string = s.to_string();
    /// ```
    pub fn to_string(&self) -> String {
        self.value.clone()
    }

    /// Gets the length of the string.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let s = StringB::new("hello");
    /// assert_eq!(s.len(), 5);
    /// ```
    pub fn len(&self) -> usize {
        self.as_bytes().len()
    }

    /// Returns `true` if the string is empty.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let s = StringB::new("");
    /// assert!(s.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Converts the string to uppercase.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let s = StringB::new("hello");
    /// assert_eq!(s.to_uppercase().as_str(), "HELLO");
    /// ```
    pub fn to_uppercase(&self) -> Self {
        let upper_str = self.as_str().to_uppercase();
        StringB::new(upper_str)
    }

    /// Converts the string to lowercase.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let s = StringB::new("HELLO");
    /// assert_eq!(s.to_lowercase().as_str(), "hello");
    /// ```
    pub fn to_lowercase(&self) -> Self {
        let lower_str = self.as_str().to_lowercase();
        StringB::new(lower_str)
    }

    /// Removes whitespace at the beginning and end of the string.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let s = StringB::new("  hello  ");
    /// assert_eq!(s.trim().as_str(), "hello");
    /// ```
    pub fn trim(&self) -> Self {
        let trimmed_str = self.as_str().trim();
        StringB::new(trimmed_str)
    }

    /// Replaces all occurrences of 'from' with 'to'.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let s = StringB::new("hello world");
    /// assert_eq!(s.replace("world", "planet").as_str(), "hello planet");
    /// ```
    pub fn replace(&self, from: &str, to: &str) -> Self {
        let replaced_str = self.as_str().replace(from, to);
        StringB::new(replaced_str)
    }

    /// Concatenates the current string with another string or `&str`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let s1 = StringB::new("hello");
    /// let s2 = " world";
    /// assert_eq!(s1.concat(s2).as_str(), "hello world");
    /// ```
    pub fn concat<T: AsRef<str>>(&self, other: T) -> Self {
        let mut result = String::from(self.as_str());
        result.push_str(other.as_ref());
        StringB::new(result)
    }

    #[cfg(feature = "cstring")]
    pub fn to_string_lossy(&self) -> String {
        self.value.to_string_lossy().into_owned()
    }

    pub fn to_string_lossy(&self) -> String {
        self.value.clone()
    }

    /// Creates a new `StringB` from a `Vec<u8>`, assuming it is valid UTF-8.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let bytes = vec![104, 101, 108, 108, 111]; // "hello" in UTF-8
    /// let s = StringB::from_utf8(bytes);
    /// ```
    pub fn from_utf8(value: Vec<u8>) -> Self {
        StringB::new(String::from_utf8(value).unwrap())
    }

    #[cfg(feature = "cstring")]
    pub fn from_utf8(value: Vec<u8>) -> Result<Self, FromUtf8Error> {
        let c_string = CString::new(value)?;
        let string = c_string.into_string()?;
        Ok(StringB::new(string))
    }
}

/// Implements the `Display` trait for `StringB`.
///
/// This allows `StringB` instances to be formatted using the `{}` placeholder in format strings.
impl Display for StringB {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string_lossy())
    }
}

/// Implements the `Deref` trait for `StringB`.
///
/// This allows treating a `StringB` instance as if it were a slice of bytes (`[u8]`).
impl Deref for StringB {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.as_bytes()
    }
}

/// Implements the `From<String>` trait for `StringB`.
///
/// This allows creating a `StringB` instance from a `String`.
impl From<String> for StringB {
    fn from(value: String) -> Self {
        StringB::new(value)
    }
}

/// Implements the `From<&str>` trait for `StringB`.
///
/// This allows creating a `StringB` instance from a `&str`.
impl From<&str> for StringB {
    fn from(value: &str) -> Self {
        StringB::new(value)
    }
}

/// Implements the `From<&Vec<u8>>` trait for `StringB`.
///
/// This allows creating a `StringB` instance from a reference to a `Vec<u8>`.
impl From<&Vec<u8>> for StringB {
    fn from(value: &Vec<u8>) -> Self {
        StringB::from_utf8(value.clone())
    }
}

/// Implements the `From<Vec<u8>>` trait for `StringB`.
///
/// This allows creating a `StringB` instance from a `Vec<u8>`.
impl From<Vec<u8>> for StringB {
    fn from(value: Vec<u8>) -> Self {
        StringB::from_utf8(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_len() {
        let s = StringB::new("Hello");
        assert_eq!(s.len(), 5);
    }

    #[test]
    fn test_is_empty() {
        let s = StringB::new("");
        assert!(s.is_empty());
    }

    #[test]
    fn test_to_uppercase() {
        let s = StringB::new("hello");
        assert_eq!(s.to_uppercase().as_str(), "HELLO");
    }

    #[test]
    fn test_to_lowercase() {
        let s = StringB::new("HELLO");
        assert_eq!(s.to_lowercase().as_str(), "hello");
    }

    #[test]
    fn test_trim() {
        let s = StringB::new("  hello  ");
        assert_eq!(s.trim().as_str(), "hello");
    }

    #[test]
    fn test_replace() {
        let s = StringB::new("hello world");
        assert_eq!(s.replace("world", "planet").as_str(), "hello planet");
    }

    #[test]
    fn test_concat() {
        let s1 = StringB::new("hello");
        let s2 = " world";
        assert_eq!(s1.concat(s2).as_str(), "hello world");
    }
}
