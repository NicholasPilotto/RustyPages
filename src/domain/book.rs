use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Book {
    pub id: uuid::Uuid,
}