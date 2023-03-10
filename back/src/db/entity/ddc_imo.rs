use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "ddc_imo")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i32,
    #[sea_orm(column_type = "Text")]
    pub value: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::ddc_imo_sdn::Entity")]
    DdcBicSdn,
}

impl Related<super::ddc_imo_sdn::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DdcBicSdn.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
