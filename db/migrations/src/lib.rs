pub use sea_orm_migration::prelude::*;

mod m20221203_181039_create_table_s3profile;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20221203_181039_create_table_s3profile::Migration)]
    }
}
