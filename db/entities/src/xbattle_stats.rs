use sea_orm::entity::prelude::*;

#[derive(
    Debug,
    Clone,
    Copy,
    EnumIter,
    PartialEq,
    Eq,
    DeriveActiveEnum,
    serde::Deserialize,
    serde::Serialize,
)]
#[sea_orm(rs_type = "u8", db_type = "TinyUnsigned")]
/// A Citizen's X Division, if applicable.
pub enum XBattleDivision {
    /// Takaroka Division.
    ///
    /// Covers citizens from Japan, HK, South Korea, Australia, and New Zealand.
    Takaroka = 0,
    /// Tentatek Division.
    ///
    /// Covers citizens from The Americas & Europe.
    Tentatek = 1,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Deserialize, serde::Serialize)]
#[sea_orm(table_name = "xbattle_stats")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(skip)]
    pub id: u64,
    pub division: XBattleDivision,
    pub splat_zones: u16,
    pub tower_control: u16,
    pub rainmaker: u16,
    pub clam_blitz: u16,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::s3_profile::Entity",
        from = "Column::Id",
        to = "super::s3_profile::Column::Id",
        on_delete = "Cascade"
    )]
    S3Profile,
}

impl Related<super::s3_profile::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::S3Profile.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
