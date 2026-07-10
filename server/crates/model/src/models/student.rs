use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Student {
    pub id: i32,
    pub name: String,
}
