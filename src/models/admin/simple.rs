use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SimpleModel {
    pub number: i32,
    pub value: String,
}
