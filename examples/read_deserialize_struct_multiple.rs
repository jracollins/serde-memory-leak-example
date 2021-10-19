use memory_leak::{ExampleEnum, ExampleStructA, ExampleStructB};
use std::fs::{read_dir, read_to_string};
use std::path::{Path, PathBuf};
use std::{thread, time::Duration};

fn read_file(path: &PathBuf) {
    println!("Starting Read for path: {:?}", path);
    let mut s = read_to_string(path).unwrap();
    let mut file: Vec<ExampleStructA> = serde_json::from_str(&s).unwrap();
    println!("Json file read, {} items", file.len());
}

fn main() {
    println!("Starting Multipler Read");

    let multiple_directory = read_dir("./data/multiple").unwrap();

    let example_paths = multiple_directory
        .filter_map(|entry| {
            entry
                .ok()
                .filter(|e| e.metadata().unwrap().is_file())
                .filter(|e| e.path().extension().unwrap() == "json")
                .map(|e| e.path())
        })
        .collect::<Vec<PathBuf>>();

    for path in example_paths {
        read_file(&path)
    }

    println!("Sleeping");

    thread::sleep(Duration::from_secs(60));

    println!("End");
}
