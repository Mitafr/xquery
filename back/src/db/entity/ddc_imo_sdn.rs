use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "ddc_imo_sdn")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub ddc_imo_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub sdn_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::ddc_imo::Entity",
        from = "Column::DdcImoId",
        to = "super::ddc_imo::Column::Id",
        on_update = "Restrict",
        on_delete = "Restrict"
    )]
    DdcImo,
    #[sea_orm(
        belongs_to = "super::sdn::Entity",
        from = "Column::SdnId",
        to = "super::sdn::Column::RecordId",
        on_update = "Restrict",
        on_delete = "Restrict"
    )]
    Sdn,
}

impl Related<super::ddc_imo::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DdcImo.def()
    }
}

impl Related<super::sdn::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Sdn.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug)]
pub struct SdnToDdcImo;

impl Linked for SdnToDdcImo {
    type FromEntity = super::sdn::Entity;

    type ToEntity = super::ddc_imo::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![
            Relation::Sdn.def().rev(),
            Entity::belongs_to(super::ddc_imo::Entity)
                .from(Column::DdcImoId)
                .to(super::ddc_imo::Column::Id)
                .into(),
        ]
    }
}
