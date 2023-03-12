use sea_orm::{entity::prelude::*, EntityTrait, RelationTrait};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Default)]
#[sea_orm(table_name = "website")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "Text")]
    pub website: String,
    #[sea_orm(column_type = "Text")]
    pub topmaj: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::website_identity::Entity")]
    WebsiteIdentity,
}

impl Related<super::website_identity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::WebsiteIdentity.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
