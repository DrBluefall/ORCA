use sea_orm_migration::{prelude::*, sea_orm::Schema};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
#[iden = "s3_profile"]
struct S3ProfileTableIdent;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let builder = manager.get_database_backend();
        let schema = Schema::new(builder);

        manager
            .create_table(schema.create_table_from_entity(entity::s3_profile::Entity))
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(S3ProfileTableIdent)
                    .cascade()
                    .to_owned(),
            )
            .await
    }
}
