use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ExampleEnum {
    A(ExampleStructA),
    B(ExampleStructB),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ExampleStructA {
    pub name_a: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ExampleStructB {
    pub name_b: String,
}
