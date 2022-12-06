use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
enum S3Profile {
    Table,
    Id,
    #[iden = "ign"]
    InGameName,
    Discriminator,
    Level,
    TurfInked,
    TotalWins,
    AnarchyRankCurrent,
    AnarchyRankBest,
    FriendCode,
    #[iden = "fclink_token"]
    FCLinkToken,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(S3Profile::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(S3Profile::Id)
                            .big_unsigned()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(S3Profile::InGameName)
                            .string_len(10)
                            .not_null()
                            .default("Player"),
                    )
                    .col(
                        ColumnDef::new(S3Profile::Discriminator)
                            .string_len(4)
                            .not_null()
                            .default("0000"),
                    )
                    .col(
                        ColumnDef::new(S3Profile::Level)
                            .small_unsigned()
                            .not_null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(S3Profile::TurfInked)
                            .unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(S3Profile::TotalWins)
                            .small_unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(S3Profile::AnarchyRankBest)
                            .string_len(4)
                            .not_null()
                            .default("C-"),
                    )
                    .col(
                        ColumnDef::new(S3Profile::AnarchyRankCurrent)
                            .string_len(4)
                            .not_null()
                            .default("C-"),
                    )
                    .col(
                        ColumnDef::new(S3Profile::FriendCode)
                            .string_len(12)
                            .not_null()
                            .default("000000000000"),
                    )
                    .col(ColumnDef::new(S3Profile::FCLinkToken).string_len(10).null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(S3Profile::Table).to_owned())
            .await
    }
}
