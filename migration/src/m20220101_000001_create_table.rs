use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AuthUsers::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AuthUsers::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(AuthUsers::Username).string().not_null())
                    .col(ColumnDef::new(AuthUsers::Password).string().not_null())
                    .col(ColumnDef::new(AuthUsers::Email).string().not_null())
                    .col(ColumnDef::new(AuthUsers::Phone).string().not_null())
                    .col(
                        ColumnDef::new(AuthUsers::Active)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(ColumnDef::new(AuthUsers::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(AuthUsers::UpdatedAt).date_time().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AuthUsers::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum AuthUsers {
    Table,
    Id,
    Username,
    Password,
    Email,
    Phone,
    Active,
    CreatedAt,
    UpdatedAt,
}
