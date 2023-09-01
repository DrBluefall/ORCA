use sea_orm::prelude::*;
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Deserialize)]
#[sea_orm(table_name = "orcasite_account")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub email: String,
    pub username: String,

    // These fields get skipped during serialization. Don't wanna accidentally
    // leak this stuff!
    #[serde(skip)]
    pub passhash_str: String,
}

#[derive(Clone, Copy, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "crate::s3_profile::Entity")]
    S3Profile,
}

impl Related<crate::s3_profile::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::S3Profile.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
