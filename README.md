# Memory Leak Example

Simple 

- Run create data `cargo run --example write_data_enum` or `cargo run --example write_data_struct` 
- Run example read string and monitor memory usage `cargo run --example read_string`
- Run deserialize to json example `cargo run --example read_deserialize_enum` or `cargo run --example read_deserialize_struct`

Examples will print out various stages and sleep for 60sc between to allow memory usage to be looked at before quitting.