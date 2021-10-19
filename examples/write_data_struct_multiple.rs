use memory_leak::{ExampleEnum, ExampleStructA};
use std::fs::{create_dir_all, File};
use std::path::{Path, PathBuf};
use std::{thread, time::Duration};

fn main() {
    println!("Starting Multiple Write For Struct");

    let number_of_files = 20;

    for i in 0..number_of_files {
        println!("Starting write {} of {}", i, number_of_files);

        let random_item_count = 1000000;
        let charset = "Abcdefghijklmnopqrstuvwxyz";

        let mut random_data: Vec<ExampleStructA> = Vec::with_capacity(random_item_count);

        for i in 0..random_item_count {
            let random_string = generate(100, charset);
            random_data.push(ExampleStructA {
                name_a: random_string,
            });
        }

        let path = format!("./data/multiple/example_struct-{}.json", i);

        let path = Path::new(&path);
        let path_parent = path.parent().unwrap();

        create_dir_all(&path_parent).unwrap();
        serde_json::to_writer(&File::create(path).unwrap(), &random_data).unwrap();
    }

    println!("Finished Write for Struct");
}

// Quickly taken from: https://github.com/DmitrijVC/random-string/blob/master/src/generator.rs
pub fn generate<S: AsRef<str>>(length: usize, charset: S) -> String {
    let charset_str = charset.as_ref();

    if charset_str.is_empty() {
        panic!("Provided charset is empty! It should contain at least one character");
    }

    let chars: Vec<char> = charset_str.chars().collect();
    let mut result = String::with_capacity(length);

    unsafe {
        for _ in 0..length {
            result.push(*chars.get_unchecked(fastrand::usize(0..chars.len())));
        }
    }

    result
}
