use value_derive::{ToValueTrait, FromValueTrait, ToJsonTrait};
use value::Value;

#[derive(Debug, PartialEq, ToValueTrait, FromValueTrait, ToJsonTrait)]
struct Person {
    name: String,
    age: u32,
}

#[test]
fn test_to_value() {
    let person = Person {
        name: "John Doe".to_string(),
        age: 30,
    };

    let value = person.to_value();
    let expected = Value::Object(
        vec![
            ("name".to_owned(), Value::String("John Doe".to_owned())),
            ("age".to_owned(), Value::Number(30.into())),
        ].into_iter().collect()
    );

    assert_eq!(value, expected);
}

#[test]
fn test_from_value() {
    let value = Value::Object(
        vec![
            ("name".to_owned(), Value::String("John Doe".to_owned())),
            ("age".to_owned(), Value::Number(30.into())),
        ].into_iter().collect()
    );

    let person = Person::from_value(value).unwrap();
    let expected = Person {
        name: "John Doe".to_string(),
        age: 30,
    };

    assert_eq!(person, expected);
}

#[test]
fn test_to_json() {
    let person = Person {
        name: "John Doe".to_string(),
        age: 30,
    };

    let json = person.to_json();
    let expected = r#"{"name":"John Doe","age":30}"#;

    assert_eq!(json, expected);
}