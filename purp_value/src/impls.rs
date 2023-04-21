use crate::prelude::*;

impl Value {
    pub fn get<T>(&self, key: T) -> Option<&Value>
    where
        T: ValueKeyBehavior,
    {
        match self {
            Value::Object(object) => object.get(key),
            _ => None,
        }
    }

    pub fn clean(&mut self) {
        match self {
            // Value::Array(array) => array.clean(),
            Value::Object(object) => {
                object.clean();
            }
            Value::Number(number) => {
                number.clean();
            }
            _ => (),
        };
    }
}

impl NumberBehavior for Value {
    fn set_u8(&mut self, value: u8) {
        match self {
            Value::Number(n) => n.set_u8(value),
            _ => (),
        }
    }

    fn set_u16(&mut self, value: u16) {
        match self {
            Value::Number(n) => n.set_u16(value),
            _ => (),
        }
    }

    fn set_u32(&mut self, value: u32) {
        match self {
            Value::Number(n) => n.set_u32(value),
            _ => (),
        }
    }

    fn set_u64(&mut self, value: u64) {
        match self {
            Value::Number(n) => n.set_u64(value),
            _ => (),
        }
    }

    fn set_u128(&mut self, value: u128) {
        match self {
            Value::Number(n) => n.set_u128(value),
            _ => (),
        }
    }

    fn set_i8(&mut self, value: i8) {
        match self {
            Value::Number(n) => n.set_i8(value),
            _ => (),
        }
    }

    fn set_i16(&mut self, value: i16) {
        match self {
            Value::Number(n) => n.set_i16(value),
            _ => (),
        }
    }

    fn set_i32(&mut self, value: i32) {
        match self {
            Value::Number(n) => n.set_i32(value),
            _ => (),
        }
    }

    fn set_i64(&mut self, value: i64) {
        match self {
            Value::Number(n) => n.set_i64(value),
            _ => (),
        }
    }

    fn set_i128(&mut self, value: i128) {
        match self {
            Value::Number(n) => n.set_i128(value),
            _ => (),
        }
    }

    fn set_f32(&mut self, value: f32) {
        match self {
            Value::Number(n) => n.set_f32(value),
            _ => (),
        }
    }

    fn set_f64(&mut self, value: f64) {
        match self {
            Value::Number(n) => n.set_f64(value),
            _ => (),
        }
    }

    fn get_u8(&self) -> Option<u8> {
        match self {
            Value::Number(n) => n.get_u8(),
            _ => None,
        }
    }

    fn get_u16(&self) -> Option<u16> {
        match self {
            Value::Number(n) => n.get_u16(),
            _ => None,
        }
    }

    fn get_u32(&self) -> Option<u32> {
        match self {
            Value::Number(n) => n.get_u32(),
            _ => None,
        }
    }

    fn get_u64(&self) -> Option<u64> {
        match self {
            Value::Number(n) => n.get_u64(),
            _ => None,
        }
    }

    fn get_u128(&self) -> Option<u128> {
        match self {
            Value::Number(n) => n.get_u128(),
            _ => None,
        }
    }

    fn get_i8(&self) -> Option<i8> {
        match self {
            Value::Number(n) => n.get_i8(),
            _ => None,
        }
    }

    fn get_i16(&self) -> Option<i16> {
        match self {
            Value::Number(n) => n.get_i16(),
            _ => None,
        }
    }

    fn get_i32(&self) -> Option<i32> {
        match self {
            Value::Number(n) => n.get_i32(),
            _ => None,
        }
    }

    fn get_i64(&self) -> Option<i64> {
        match self {
            Value::Number(n) => n.get_i64(),
            _ => None,
        }
    }

    fn get_i128(&self) -> Option<i128> {
        match self {
            Value::Number(n) => n.get_i128(),
            _ => None,
        }
    }

    fn get_f32(&self) -> Option<f32> {
        match self {
            Value::Number(n) => n.get_f32(),
            _ => None,
        }
    }

    fn get_f64(&self) -> Option<f64> {
        match self {
            Value::Number(n) => n.get_f64(),
            _ => None,
        }
    }

    fn get_u8_unsafe(&self) -> u8 {
        match self {
            Value::Number(n) => n.get_u8_unsafe(),
            _ => todo!(),
        }
    }

    fn get_u16_unsafe(&self) -> u16 {
        match self {
            Value::Number(n) => n.get_u16_unsafe(),
            _ => todo!(),
        }
    }

    fn get_u32_unsafe(&self) -> u32 {
        match self {
            Value::Number(n) => n.get_u32_unsafe(),
            _ => todo!(),
        }
    }

    fn get_u64_unsafe(&self) -> u64 {
        match self {
            Value::Number(n) => n.get_u64_unsafe(),
            _ => todo!(),
        }
    }

    fn get_u128_unsafe(&self) -> u128 {
        match self {
            Value::Number(n) => n.get_u128_unsafe(),
            _ => todo!(),
        }
    }

    fn get_i8_unsafe(&self) -> i8 {
        match self {
            Value::Number(n) => n.get_i8_unsafe(),
            _ => todo!(),
        }
    }

    fn get_i16_unsafe(&self) -> i16 {
        match self {
            Value::Number(n) => n.get_i16_unsafe(),
            _ => todo!(),
        }
    }

    fn get_i32_unsafe(&self) -> i32 {
        match self {
            Value::Number(n) => n.get_i32_unsafe(),
            _ => todo!(),
        }
    }

    fn get_i64_unsafe(&self) -> i64 {
        match self {
            Value::Number(n) => n.get_i64_unsafe(),
            _ => todo!(),
        }
    }

    fn get_i128_unsafe(&self) -> i128 {
        match self {
            Value::Number(n) => n.get_i128_unsafe(),
            _ => todo!(),
        }
    }

    fn get_f32_unsafe(&self) -> f32 {
        match self {
            Value::Number(n) => n.get_f32_unsafe(),
            _ => todo!(),
        }
    }

    fn get_f64_unsafe(&self) -> f64 {
        match self {
            Value::Number(n) => n.get_f64_unsafe(),
            _ => todo!(),
        }
    }

    fn is_i8(&self) -> bool {
        match self {
            Value::Number(n) => n.is_i8(),
            _ => false,
        }
    }

    fn is_i16(&self) -> bool {
        match self {
            Value::Number(n) => n.is_i16(),
            _ => false,
        }
    }

    fn is_i32(&self) -> bool {
        match self {
            Value::Number(n) => n.is_i32(),
            _ => false,
        }
    }

    fn is_i64(&self) -> bool {
        match self {
            Value::Number(n) => n.is_i64(),
            _ => false,
        }
    }

    fn is_i128(&self) -> bool {
        match self {
            Value::Number(n) => n.is_i128(),
            _ => false,
        }
    }

    fn is_u8(&self) -> bool {
        match self {
            Value::Number(n) => n.is_u8(),
            _ => false,
        }
    }

    fn is_u16(&self) -> bool {
        match self {
            Value::Number(n) => n.is_u16(),
            _ => false,
        }
    }

    fn is_u32(&self) -> bool {
        match self {
            Value::Number(n) => n.is_u32(),
            _ => false,
        }
    }

    fn is_u64(&self) -> bool {
        match self {
            Value::Number(n) => n.is_u64(),
            _ => false,
        }
    }

    fn is_u128(&self) -> bool {
        match self {
            Value::Number(n) => n.is_u128(),
            _ => false,
        }
    }

    fn is_f32(&self) -> bool {
        match self {
            Value::Number(n) => n.is_f32(),
            _ => false,
        }
    }

    fn is_f64(&self) -> bool {
        match self {
            Value::Number(n) => n.is_f64(),
            _ => false,
        }
    }

    fn is_number(&self) -> bool {
        match self {
            Value::Number(_) => true,
            _ => false,
        }
    }

    fn is_integer(&self) -> bool {
        match self {
            Value::Number(n) => n.is_integer(),
            _ => false,
        }
    }

    fn is_float(&self) -> bool {
        match self {
            Value::Number(n) => n.is_float(),
            _ => false,
        }
    }

    fn is_signed(&self) -> bool {
        match self {
            Value::Number(n) => n.is_signed(),
            _ => false,
        }
    }

    fn is_unsigned(&self) -> bool {
        match self {
            Value::Number(n) => n.is_unsigned(),
            _ => false,
        }
    }

    fn is_zero(&self) -> bool {
        match self {
            Value::Number(n) => n.is_zero(),
            _ => false,
        }
    }

    fn is_positive(&self) -> bool {
        match self {
            Value::Number(n) => n.is_positive(),
            _ => false,
        }
    }

    fn is_negative(&self) -> bool {
        match self {
            Value::Number(n) => n.is_negative(),
            _ => false,
        }
    }

    fn number_type(&self) -> NumberType {
        match self {
            Value::Number(n) => n.number_type(),
            _ => NumberType::Unknown,
        }
    }
}

impl ObjectBehavior for Value {
    fn insert<T>(&mut self, key: T, value: Value) -> Option<Value>
    where
        T: ValueKeyBehavior,
    {
        match self {
            Value::Object(o) => o.insert(key, value),
            _ => todo!(),
        }
    }

    fn remove<T>(&mut self, key: &T) -> Option<Value>
    where
        T: ValueKeyBehavior,
    {
        match self {
            Value::Object(o) => o.remove(key),
            _ => todo!(),
        }
    }

    fn contains_key<T>(&self, key: &T) -> bool
    where
        T: ValueKeyBehavior,
    {
        match self {
            Value::Object(o) => o.contains_key(key),
            _ => todo!(),
        }
    }

    fn keys(&self) -> Vec<&ValueKey> {
        match self {
            Value::Object(o) => o.keys(),
            _ => todo!(),
        }
    }

    fn values(&self) -> Vec<&Value> {
        match self {
            Value::Object(o) => o.values(),
            _ => todo!(),
        }
    }

    fn len(&self) -> usize {
        match self {
            Value::Object(o) => o.len(),
            _ => todo!(),
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            Value::Object(o) => o.is_empty(),
            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::prelude::*;

    #[test]
    fn test_value_number_behavior() {
        let value = Value::from(3.14);
        assert_eq!(value.get_f64_unsafe(), 3.14);
    }
    #[test]
    fn test_value_object_behavior() {
        // let mut value = Value::from(HashMap::from_iter(
        //     vec![("1".to_string(), Value::from(3.14))].into_iter(),
        // ));
        // value.insert("2".to_string(), 4.13);
        // assert_eq!(value, 3.14);
    }
}
