use sea_orm::entity::prelude::*;

use crate::db::OfacRelEntity;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "aircraft_identity")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub aircraft_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub identity_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "super::aircraft::Entity", from = "Column::AircraftId", to = "super::aircraft::Column::Id", on_update = "Restrict", on_delete = "Restrict")]
    Aircraft,
    #[sea_orm(belongs_to = "super::sdn::Entity", from = "Column::IdentityId", to = "super::sdn::Column::Identity", on_update = "Restrict", on_delete = "Restrict")]
    Sdn,
}

impl Related<super::aircraft::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Aircraft.def()
    }
}

impl Related<super::sdn::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Sdn.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl OfacRelEntity for ActiveModel {}

#[derive(Debug)]
pub struct SdnToAircraft;

impl Linked for SdnToAircraft {
    type FromEntity = super::sdn::Entity;

    type ToEntity = super::aircraft::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![
            Relation::Sdn.def().rev(),
            Entity::belongs_to(super::aircraft::Entity).from(Column::AircraftId).to(super::aircraft::Column::Id).into(),
        ]
    }
}
