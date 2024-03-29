pub use sea_orm_migration::prelude::*;

mod m20221203_181039_create_table_s3profile;
mod m20221207_224215_create_table_xbattle_stats;
mod m20230829_065706_orcasite_account;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230829_065706_orcasite_account::Migration),
            Box::new(m20221203_181039_create_table_s3profile::Migration),
            Box::new(m20221207_224215_create_table_xbattle_stats::Migration),
        ]
    }
}
