// to serialize to json we need serde_json and serde
// add by cargo add serde --features derive, cargo add serde_json

#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8
}

pub fn serialize_example() {
    let person = Person {
        name: "Alice".to_string(),
        age: 30
    };

    let json_str = serde_json::to_string(&person).unwrap();
    println!("json_str: {:?}", json_str);

    let pretty_json_str = serde_json::to_string_pretty(&person).unwrap();
    println!("pretty_json_str: {:?}", pretty_json_str);

    let deserialized = serde_json::from_str::<Person>(&json_str).unwrap();
    println!("deserialized: {:?}", deserialized);
}