use sea_orm::{entity::prelude::*, EntityTrait, RelationTrait};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Default)]
#[sea_orm(table_name = "nationality_registration")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i32,
    #[sea_orm(column_type = "Text", nullable)]
    pub location: Option<String>,
    #[sea_orm(column_type = "custom(\"TINYTEXT\")")]
    pub topmaj: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::nationality_registration_sdn::Entity")]
    NationalityRegistrationSdn,
}

impl Related<super::nationality_registration_sdn::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::NationalityRegistrationSdn.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
