use crate::prelude::*;

impl Value {
    pub fn get<T>(&self, key: T) -> Option<&Value>
    where
        T: ValueKeyBehavior,
    {
        match self {
            Value::Object(object) => object.get(key),
            Value::Array(array) => {
                let key = key.to_value_key();
                array.get(key.to_usize())
            }
            _ => None,
        }
    }

    pub fn get_mut<T>(&mut self, key: T) -> Option<&mut Value>
    where
        T: ValueKeyBehavior,
    {
        match self {
            Value::Object(object) => object.get_mut(key),
            Value::Array(array) => {
                let key = key.to_value_key();
                array.get_mut(key.to_usize())
            }
            _ => None,
        }
    }

    pub fn clean(&mut self) {
        match self {
            Value::Array(array) => array.clean(),
            Value::Object(object) => {
                object.clean();
            }
            Value::Number(number) => {
                number.clean();
            }
            _ => (),
        };
    }

    fn len(&self) -> usize {
        match self {
            Value::Array(array) => array.len(),
            Value::Object(object) => object.len(),
            _ => 0,
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            Value::Array(array) => array.is_empty(),
            Value::Object(object) => object.is_empty(),
            _ => true,
        }
    }

    fn push<T: ToValueBehavior>(&mut self, value: T) {
        match self {
            Value::Array(array) => array.push(value.to_value()),
            _ => (),
        }
    }

    fn insert<T, V>(&mut self, key: T, value: V) -> Option<Value>
    where
        T: ValueKeyBehavior,
        V: ToValueBehavior,
    {
        match self {
            Value::Object(o) => o.insert(key, value.to_value()),
            _ => todo!(),
        }
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
}

impl ArrayBehavior for Value {
    fn pop(&mut self) -> Option<Value> {
        match self {
            Value::Array(array) => array.pop(),
            _ => None,
        }
    }
}

impl DateTimeBehavior for Value {
    fn as_date(&self) -> Option<&chrono::NaiveDate> {
        match self {
            Value::DateTime(datetime) => datetime.as_date(),
            _ => None,
        }
    }

    fn as_time(&self) -> Option<&chrono::NaiveTime> {
        match self {
            Value::DateTime(datetime) => datetime.as_time(),
            _ => None,
        }
    }

    fn as_date_time(&self) -> Option<&chrono::DateTime<chrono::Utc>> {
        match self {
            Value::DateTime(datetime) => datetime.as_date_time(),
            _ => None,
        }
    }

    fn year(&self) -> Option<i32> {
        match self {
            Value::DateTime(datetime) => datetime.year(),
            _ => None,
        }
    }

    fn month(&self) -> Option<u32> {
        match self {
            Value::DateTime(datetime) => datetime.month(),
            _ => None,
        }
    }

    fn day(&self) -> Option<u32> {
        match self {
            Value::DateTime(datetime) => datetime.day(),
            _ => None,
        }
    }

    fn hour(&self) -> Option<u32> {
        match self {
            Value::DateTime(datetime) => datetime.hour(),
            _ => None,
        }
    }

    fn minute(&self) -> Option<u32> {
        match self {
            Value::DateTime(datetime) => datetime.minute(),
            _ => None,
        }
    }

    fn second(&self) -> Option<u32> {
        match self {
            Value::DateTime(datetime) => datetime.second(),
            _ => None,
        }
    }

    fn timestamp(&self) -> Option<i64> {
        match self {
            Value::DateTime(datetime) => datetime.timestamp(),
            _ => None,
        }
    }

    fn timezone(&self) -> Option<chrono::Utc> {
        match self {
            Value::DateTime(datetime) => datetime.timezone(),
            _ => None,
        }
    }

    fn to_iso8601(&self) -> String {
        match self {
            Value::DateTime(datetime) => datetime.to_iso8601(),
            _ => "".to_string(),
        }
    }

    fn to_rfc3339(&self) -> String {
        match self {
            Value::DateTime(datetime) => datetime.to_rfc3339(),
            _ => "".to_string(),
        }
    }

    fn add_duration(&self, duration: chrono::Duration) -> Option<DateTime>
    where
        Self: Sized,
    {
        match self {
            Value::DateTime(datetime) => datetime.add_duration(duration),
            _ => None,
        }
    }

    fn subtract_duration(&self, duration: chrono::Duration) -> Option<DateTime>
    where
        Self: Sized,
    {
        match self {
            Value::DateTime(datetime) => datetime.subtract_duration(duration),
            _ => None,
        }
    }

    fn duration_between(&self, other: &DateTime) -> Option<chrono::Duration> {
        match self {
            Value::DateTime(datetime) => datetime.duration_between(other),
            _ => None,
        }
    }

    fn from_ymd_opt(year: i32, month: u32, day: u32) -> DateTime {
        DateTime::from_ymd_opt(year, month, day)
    }

    fn with_ymd_and_hms(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        min: u32,
        sec: u32,
    ) -> DateTime {
        DateTime::with_ymd_and_hms(year, month, day, hour, min, sec)
    }

    fn now() -> DateTime {
        DateTime::now()
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use std::collections::HashMap;

    #[test]
    fn test_value_number_behavior() {
        let value = Value::from(3.14);
        assert_eq!(value.get_f64_unsafe(), 3.14);
    }

    #[test]
    fn test_value_object_behavior() {
        let mut value = Value::from(HashMap::from_iter(vec![("1", 3.14.to_value())].into_iter()));
        value.insert("2", 4.13);

        if let Some(item) = value.get_mut("1") {
            *item = 1.43.to_value();
        }

        assert_eq!(value.get("1").unwrap(), &1.43.to_value());
    }

    #[test]
    fn test_value_array_behavior() {
        let mut value = Value::from(vec![1, 2, 3]);
        value.push(4);

        if let Some(item) = value.get_mut("1") {
            *item = 1.43.to_value();
        }

        assert_eq!(value.get("1").unwrap(), &1.43.to_value());
    }

    #[test]
    fn test_value_datetime_behavior() {
        let dt_date = Value::from_ymd_opt(2023, 4, 5);
        let dt_datetime = Value::with_ymd_and_hms(2023, 4, 5, 12, 34, 56);

        assert_eq!(
            dt_date.add_duration(Duration::days(1)),
            Some(DateTime::from(NaiveDate::from_ymd_opt(2023, 4, 6).unwrap()))
        );
        assert_eq!(
            dt_datetime.add_duration(Duration::days(1)),
            Some(DateTime::from(Utc.with_ymd_and_hms(2023, 4, 6, 12, 34, 56)))
        );
    }
}
