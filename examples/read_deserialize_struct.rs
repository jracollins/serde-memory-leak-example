use memory_leak::{ExampleEnum, ExampleStructA, ExampleStructB};
use std::fs::read_to_string;
use std::{thread, time::Duration};

fn main() {
    println!("Starting Read String for Struct");
    let mut s = read_to_string("./data/example_struct.json").unwrap();

    let mut file: Vec<ExampleStructA> = serde_json::from_str(&s).unwrap();
    println!("Json file read, {} items, now sleeping...", file.len());
    s = String::from("");

    thread::sleep(Duration::from_secs(30));

    println!("Dropping Reference");

    file = Vec::new();

    println!("Reference Dropped, file items {}", file.len());

    thread::sleep(Duration::from_secs(30));

    println!("Re-Reading String for Struct");

    s = read_to_string("./data/example_struct.json").unwrap();

    file = serde_json::from_str(&s).unwrap();
    println!("Json file read, {} items, now sleeping...", file.len());
    s = String::from("");

    thread::sleep(Duration::from_secs(30));

    println!("Reference Dropped, file items {}", file.len());

    thread::sleep(Duration::from_secs(30));

    println!("End");
}
