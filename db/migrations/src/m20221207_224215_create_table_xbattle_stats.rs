use entity::xbattle_stats::Entity as XBattleStatsEnt;
use sea_orm_migration::{prelude::*, sea_orm::Schema};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
#[iden = "xbattle_stats"]
pub struct XBattleStatsTable;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let builder = manager.get_database_backend();
        let schema = Schema::new(builder);

        manager
            .create_table(schema.create_table_from_entity(XBattleStatsEnt))
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(
                Table::drop()
                    .table(XBattleStatsTable)
                    .if_exists()
                    .to_owned(),
            )
            .await
    }
}
