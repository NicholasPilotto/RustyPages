use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Book {
    pub id: Uuid,
}