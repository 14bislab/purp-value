use crate::traits::ToValueTrait;
use crate::Value;
use std::collections::{BTreeMap, HashMap};
use std::iter::Iterator;

/// An enum representing a JSON object as a `BTreeMap` or a `HashMap`.
#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    BTreeMap(BTreeMap<String, Value>),
    HashMap(HashMap<String, Value>),
}

impl Object {
    /// Returns a reference to the value associated with the specified key, or `None` if the key is not present.
    pub fn get(&self, key: &str) -> Option<&Value> {
        match self {
            Object::BTreeMap(map) => map.get(key),
            Object::HashMap(map) => map.get(key),
        }
    }

    /// Inserts a key-value pair into the object. If the key already exists, returns the previous value associated with the key.
    pub fn insert(&mut self, key: String, value: Value) -> Option<Value> {
        match self {
            Object::BTreeMap(map) => map.insert(key, value),
            Object::HashMap(map) => map.insert(key, value),
        }
    }

    /// Removes a key-value pair from the object and returns the associated value. If the key is not present, returns `None`.
    pub fn remove(&mut self, key: &str) -> Option<Value> {
        match self {
            Object::BTreeMap(map) => map.remove(key),
            Object::HashMap(map) => map.remove(key),
        }
    }

    /// Returns `true` if the object contains a value for the specified key, otherwise `false`.
    pub fn contains_key(&self, key: &str) -> bool {
        match self {
            Object::BTreeMap(map) => map.contains_key(key),
            Object::HashMap(map) => map.contains_key(key),
        }
    }

    /// Returns a `Vec` of references to the keys in the object, in the order they were inserted.
    pub fn keys(&self) -> Vec<&String> {
        match self {
            Object::BTreeMap(map) => map.keys().collect(),
            Object::HashMap(map) => map.keys().collect(),
        }
    }

    /// Returns a `Vec` of references to the values in the object, in the order they were inserted.
    pub fn values(&self) -> Vec<&Value> {
        match self {
            Object::BTreeMap(map) => map.values().collect(),
            Object::HashMap(map) => map.values().collect(),
        }
    }

    /// Returns the number of key-value pairs in the object.
    pub fn len(&self) -> usize {
        match self {
            Object::BTreeMap(map) => map.len(),
            Object::HashMap(map) => map.len(),
        }
    }

    /// Returns `true` if the object contains no key-value pairs, otherwise `false`.
    pub fn is_empty(&self) -> bool {
        match self {
            Object::BTreeMap(map) => map.is_empty(),
            Object::HashMap(map) => map.is_empty(),
        }
    }

    /// Removes all key-value pairs from the object.
    pub fn clear(&mut self) {
        match self {
            Object::BTreeMap(map) => map.clear(),
            Object::HashMap(map) => map.clear(),
        }
    }
}

impl Default for Object {
    /// Creates a new `Object` with an empty `HashMap`.
    fn default() -> Self {
        Object::HashMap(HashMap::new())
    }
}

impl ToValueTrait for Object {
    /// Converts Object into a Value enum.
    fn to_value(&self) -> Value {
        Value::Object(self.clone())
    }
}

impl From<BTreeMap<String, Value>> for Object {
    /// Converts BTreeMap<String, Value> into Object.
    fn from(value: BTreeMap<String, Value>) -> Self {
        Object::BTreeMap(value)
    }
}

impl From<HashMap<String, Value>> for Object {
    /// Converts HashMap<String, Value> into Object.
    fn from(value: HashMap<String, Value>) -> Self {
        Object::HashMap(value)
    }
}

impl From<Vec<(String, Value)>> for Object {
    /// Converts a vector of key-value pairs into an Object.
    fn from(value: Vec<(String, Value)>) -> Self {
        Object::HashMap(value.into_iter().collect())
    }
}

impl Into<HashMap<String, Value>> for Object {
    /// Converts Object into HashMap<String, Value>.
    fn into(self) -> HashMap<String, Value> {
        match self {
            Object::BTreeMap(map) => map.into_iter().collect(),
            Object::HashMap(map) => map,
        }
    }
}

impl Into<BTreeMap<String, Value>> for Object {
    /// Converts Object into BTreeMap<String, Value>.
    fn into(self) -> BTreeMap<String, Value> {
        match self {
            Object::BTreeMap(map) => map,
            Object::HashMap(map) => map.into_iter().collect(),
        }
    }
}

/// An iterator over the key-value pairs in an Object.
#[allow(dead_code)]
pub struct ObjectIter<'a> {
    object: &'a Object,
    state: IterState<'a>,
}

enum IterState<'a> {
    BTreeMap(std::collections::btree_map::Iter<'a, String, Value>),
    HashMap(std::collections::hash_map::Iter<'a, String, Value>),
}

impl<'a> Iterator for ObjectIter<'a> {
    type Item = (&'a String, &'a Value);

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.state {
            IterState::BTreeMap(iter) => iter.next(),
            IterState::HashMap(iter) => iter.next(),
        }
    }
}

impl<'a> Object {
    pub fn iter(&'a self) -> ObjectIter<'a> {
        match self {
            Object::BTreeMap(map) => ObjectIter {
                object: self,
                state: IterState::BTreeMap(map.iter()),
            },

            Object::HashMap(map) => ObjectIter {
                object: self,
                state: IterState::HashMap(map.iter()),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::StringB;

    use super::*;

    #[test]
    fn test_object_iter() {
        let value1 = Value::Null;
        let value2 = StringB::from("ok").to_value();

        let mut map = BTreeMap::new();
        map.insert("key1".to_string(), value1.clone());
        map.insert("key2".to_string(), value2.clone());
        let obj = Object::from(map);

        let mut iter = obj.iter();
        let mut results = vec![];

        while let Some((key, value)) = iter.next() {
            results.push((key.clone(), value.clone()));
        }

        assert_eq!(
            results,
            vec![("key1".to_string(), value1), ("key2".to_string(), value2)]
        );
    }

    #[test]
    fn test_object_from_vec() {
        let vec = vec![
            ("key1".to_string(), Value::Null),
            ("key2".to_string(), StringB::from("ok").to_value()),
        ];

        let obj = Object::from(vec);
        assert_eq!(obj.get("key1"), Some(&Value::Null));
        assert_eq!(obj.get("key2"), Some(&StringB::from("ok").to_value()));
    }
}
