use memory_leak::{ExampleEnum, ExampleStructA, ExampleStructB};
use std::fs::read_to_string;
use std::{thread, time::Duration};

fn main() {
    println!("Starting Read String");
    let mut s = read_to_string("./data/example_enum.json").unwrap();

    let mut file: Vec<ExampleEnum> = serde_json::from_str(&s).unwrap();
    println!("Json file read, {} items, now sleeping...", file.len());
    s = String::from("");

    thread::sleep(Duration::from_secs(60));

    println!("Dropping Reference");

    file = Vec::new();

    println!("Reference Dropped, file items {}", file.len());

    thread::sleep(Duration::from_secs(60));

    println!("End");
}
