use sea_orm::{entity::prelude::*, EntityTrait, RelationTrait};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Default)]
#[sea_orm(table_name = "former_vessel_flag")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "Text")]
    pub value: String,
    #[sea_orm(column_type = "custom(\"TINYTEXT\")")]
    pub topmaj: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::former_vessel_flag_sdn::Entity")]
    FormerVesselFlagSdn,
}

impl Related<super::former_vessel_flag_sdn::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FormerVesselFlagSdn.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
