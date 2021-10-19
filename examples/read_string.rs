use std::fs::read_to_string;
use std::{thread, time::Duration};
fn main() {
    println!("Starting Read String");
    let mut file_as_string = read_to_string("./data/example_enum.json").unwrap();

    println!(
        "Json file read as string, bytes: {}, now sleeping...",
        file_as_string.len()
    );

    thread::sleep(Duration::from_secs(60));

    println!("Dropping Reference");

    file_as_string = String::from("");

    println!("Reference Dropped");

    thread::sleep(Duration::from_secs(60));

    println!("End");
}
