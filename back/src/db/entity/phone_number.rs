use sea_orm::{entity::prelude::*, EntityTrait, RelationTrait};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Default)]
#[sea_orm(table_name = "phone_number")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i32,
    #[sea_orm(column_type = "Text")]
    pub phone_number: String,
    #[sea_orm(column_type = "custom(\"TINYTEXT\")")]
    pub topmaj: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::phone_number_sdn::Entity")]
    PhoneNumberSdn,
}

impl Related<super::phone_number_sdn::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PhoneNumberSdn.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
