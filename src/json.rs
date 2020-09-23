use serde::{Deserialize, Serialize};
use serde_json::{json, Result, Value};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn untyped() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;
    let v: Value = serde_json::from_str(&data)?;
    println!("Please call {} at the number {}", v["name"], v["phones"][0]);
    Ok(())
}

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

fn typed() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    let p: Person = serde_json::from_str(&data)?;
    println!(
        "Please call {} at the number {} from typed funtion.",
        p.name, p.phones[0]
    );

    Ok(())
}

/// Constructing JSON values
fn constructing() {
    // json! macro to build serde_json::Value objects with very natural JSON syntax
    // The type of `john` is `serde_json::Value`
    let john = json!({
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });

    println!("first phone number: {}", john["phones"][0]);

    // Convert to a string of JSON and print it out
    println!("{}", john.to_string());
}

#[derive(Serialize, Deserialize)]
struct Address {
    street: String,
    city: String,
}

/// Creating JSON by serializing data structures
/// Any type that implements Serde's Serialize trait can be serialized this way. This includes built-in Rust standard library types like `Vec<T>` and `HashMap<K, V>`, as well as any structs or enums annotated with `#[derive(Serialize)]`.
fn print_an_address() -> Result<()> {
    // Some data structure.
    let address = Address {
        street: "10 Downing Street".to_owned(),
        city: "London".to_owned(),
    };

    // Serialize it to a JSON string.
    let j = serde_json::to_string(&address)?;

    // Print, write to a file, or send to an HTTP server.
    println!("{}", j);

    Ok(())
}

pub fn run() {
    let point = Point { x: 1, y: 2 };

    // Convert the Point to a JSON string.build_helper
    let serialized = serde_json::to_string(&point).unwrap();
    println!("Serialized = {}", serialized);

    untyped().unwrap();
    typed().unwrap();
    constructing();

    print_an_address().unwrap();
}
