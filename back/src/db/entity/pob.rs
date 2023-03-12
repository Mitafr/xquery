use sea_orm::{entity::prelude::*, EntityTrait, RelationTrait};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Default)]
#[sea_orm(table_name = "pob")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "Text")]
    pub pob: String,
    #[sea_orm(column_type = "custom(\"TINYTEXT\")")]
    pub topmaj: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::pob_identity::Entity")]
    PobIdentity,
}

impl Related<super::pob_identity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PobIdentity.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
