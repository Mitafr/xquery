use sea_orm::{entity::prelude::*, EntityTrait, RelationTrait};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Default)]
#[sea_orm(table_name = "aircraft_operator")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i32,
    #[sea_orm(column_type = "Text")]
    pub operator: String,
    #[sea_orm(column_type = "custom(\"TINYTEXT\")")]
    pub topmaj: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::aircraft_operator_sdn::Entity")]
    AircraftOperatorSdn,
}

impl Related<super::aircraft_operator_sdn::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AircraftOperatorSdn.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
