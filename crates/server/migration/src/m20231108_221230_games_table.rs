use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Game::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Game::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Game::Title).string().not_null())
                    .col(ColumnDef::new(Game::GoogleDriveId).string().not_null())
                    .col(ColumnDef::new(Game::Img).binary().not_null())
                    .col(ColumnDef::new(Game::Exe).string().not_null())
                    .col(ColumnDef::new(Game::Hooks).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Game::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Game {
    Table,
    Id,
    Title,
    GoogleDriveId,
    Img,
    Exe,
    Hooks,
}
