use sea_orm::{entity::prelude::*, EntityTrait, RelationTrait};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Default)]
#[sea_orm(table_name = "issuer_name")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i32,
    #[sea_orm(column_type = "Text")]
    pub issuer_name: String,
    #[sea_orm(column_type = "custom(\"TINYTEXT\")")]
    pub topmaj: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::issuer_name_sdn::Entity")]
    IssuerNameSdn,
}

impl Related<super::issuer_name_sdn::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::IssuerNameSdn.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
