use sea_orm::{entity::prelude::*, EntityTrait, RelationTrait};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Default)]
#[sea_orm(table_name = "dob")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "Text")]
    pub dob: String,
    #[sea_orm(column_type = "custom(\"TINYTEXT\")")]
    pub topmaj: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::dob_identity::Entity")]
    DobIdentity,
}

impl Related<super::dob_identity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DobIdentity.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
