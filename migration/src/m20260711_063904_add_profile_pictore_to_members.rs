use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Members::Table)
                    .add_column(ColumnDef::new(Members::ProfilePicture).text())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Members::Table)
                    .drop_column(Members::ProfilePicture)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Members {
    Table,
    ProfilePicture,
}
