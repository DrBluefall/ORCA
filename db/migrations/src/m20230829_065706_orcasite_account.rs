use entity::orcasite_account::Entity as OrcaAccountEntity;
use entity::sea_orm::Schema;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
#[iden = "orcasite_account"]
pub struct OrcaAccountTable;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let builder = manager.get_database_backend();
        let schema = Schema::new(builder);
        manager
            .create_table(schema.create_table_from_entity(OrcaAccountEntity))
            .await?;
        let indexes = schema.create_index_from_entity(OrcaAccountEntity);
        for index in indexes {
            manager.create_index(index).await?;
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(OrcaAccountTable).if_exists().to_owned())
            .await
    }
}
