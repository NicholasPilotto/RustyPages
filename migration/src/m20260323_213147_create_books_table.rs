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
                    .table(Book::Table)
                    .if_not_exists()

                    // Primary key (UUID)
                    .col(
                        ColumnDef::new(Book::Id)
                            .uuid()
                            .not_null()
                            .default(Expr::cust("uuid_generate_v4()"))
                            .primary_key()
                    )

                    // Core fields
                    .col(string(Book::Title))
                    .col(text_null(Book::Description))
                    .col(string(Book::Author))

                    // Metadata
                    .col(string_null(Book::Isbn))
                    .col(date_null(Book::PublishedAt))
                    .col(integer_null(Book::Pages))
                    .col(string_null(Book::Language))

                    // Timestamps
                    .col(
                        ColumnDef::new(Book::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp())
                    )
                    .col(
                        ColumnDef::new(Book::UpdatedAt)
                            .timestamp()
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
                    .table(Book::Table)
                    .col(Book::Isbn)
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
                    .table(Book::Table)
                    .to_owned()
            )
            .await?;

        // Drop table
        manager
            .drop_table(
                Table::drop()
                    .table(Book::Table)
                    .to_owned()
            )
            .await
    }
}

#[derive(Iden)]
enum Book {
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