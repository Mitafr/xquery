use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(Log::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(entities::log::Column::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(entities::log::Column::User)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(entities::log::Column::Level)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(entities::log::Column::Description)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(entities::log::Column::CreatedAt)
                            .date_time()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Log::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Log {
    Table,
}
