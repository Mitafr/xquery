use sea_orm::entity::prelude::*;

use crate::db::OfacRelEntity;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "eo13662dd_sdn")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub eo13662dd_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub sdn_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::eo13662dd::Entity",
        from = "Column::Eo13662ddId",
        to = "super::eo13662dd::Column::Id",
        on_update = "Restrict",
        on_delete = "Restrict"
    )]
    Eo13662dd,
    #[sea_orm(
        belongs_to = "super::sdn::Entity",
        from = "Column::SdnId",
        to = "super::sdn::Column::FixedRef",
        on_update = "Restrict",
        on_delete = "Restrict"
    )]
    Sdn,
}

impl Related<super::eo13662dd::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Eo13662dd.def()
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
pub struct SdnToEo13662dd;

impl Linked for SdnToEo13662dd {
    type FromEntity = super::sdn::Entity;

    type ToEntity = super::eo13662dd::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![
            Relation::Sdn.def().rev(),
            Entity::belongs_to(super::eo13662dd::Entity)
                .from(Column::Eo13662ddId)
                .to(super::eo13662dd::Column::Id)
                .into(),
        ]
    }
}
