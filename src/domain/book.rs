use sea_orm::entity::prelude::*;
use sea_orm_migration::seaql_migrations::Relation;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, ToSchema)]
#[sea_orm(table_name = "books")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    pub title: String,
    pub description: Option<String>,

    pub author: String,
    pub isbn: Option<String>,

    #[schema(value_type = String, format = "date-time", nullable = true)]
    pub published_at: Option<DateTimeUtc>,

    pub pages: Option<i32>,
    pub language: Option<String>,

    #[schema(value_type = String, format = "date-time")]
    pub created_at: DateTimeUtc,

    #[schema(value_type = String, format = "date-time")]
    pub updated_at: DateTimeUtc,
}

impl ActiveModelBehavior for ActiveModel {}
