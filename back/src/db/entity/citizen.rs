use sea_orm::{entity::prelude::*, EntityTrait, RelationTrait};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Default)]
#[sea_orm(table_name = "citizen")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "Text", nullable)]
    pub location: Option<String>,
    #[sea_orm(column_type = "custom(\"TINYTEXT\")")]
    pub topmaj: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::citizen_sdn::Entity")]
    CitizenSdn,
}

impl Related<super::citizen_sdn::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CitizenSdn.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
