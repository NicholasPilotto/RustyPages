use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::{
    string, string_null, text_null, integer_null, date_null
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // (Optional but recommended for Postgres UUID generation)
        manager
            .get_connection()
            .execute_unprepared(
                r#"CREATE EXTENSION IF NOT EXISTS "uuid-ossp";"#
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Books::Table)
                    .if_not_exists()

                    // Primary key (UUID)
                    .col(
                        ColumnDef::new(Books::Id)
                            .uuid()
                            .not_null()
                            .default(Expr::cust("uuid_generate_v4()"))
                            .primary_key()
                    )

                    // Core fields
                    .col(string(Books::Title))
                    .col(text_null(Books::Description))
                    .col(string(Books::Author))

                    // Metadata
                    .col(string_null(Books::Isbn))
                    .col(ColumnDef::new(Books::PublishedAt)
                            .timestamp_with_time_zone()
                            .null())
                    .col(integer_null(Books::Pages))
                    .col(string_null(Books::Language))

                    // Timestamps
                    .col(
                        ColumnDef::new(Books::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp())
                    )
                    .col(
                        ColumnDef::new(Books::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp())
                    )

                    .to_owned()
            )
            .await?;

        // Index on ISBN
        manager
            .create_index(
                Index::create()
                    .name("idx-books-isbn")
                    .table(Books::Table)
                    .col(Books::Isbn)
                    .to_owned()
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop index first
        manager
            .drop_index(
                Index::drop()
                    .name("idx-books-isbn")
                    .table(Books::Table)
                    .to_owned()
            )
            .await?;

        // Drop table
        manager
            .drop_table(
                Table::drop()
                    .table(Books::Table)
                    .to_owned()
            )
            .await
    }
}

#[derive(Iden)]
enum Books {
    Table,
    Id,
    Title,
    Description,
    Author,
    Isbn,
    PublishedAt,
    Pages,
    Language,
    CreatedAt,
    UpdatedAt,
}