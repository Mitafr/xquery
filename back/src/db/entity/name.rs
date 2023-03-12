use sea_orm::{entity::prelude::*, EntityTrait, RelationTrait};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Default)]
#[sea_orm(table_name = "name")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "Text", column_name = "type")]
    pub name_type: String,
    pub script: i32,
    #[sea_orm(column_type = "Text", nullable)]
    pub last_name: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub first_name: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub middle_name: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub maiden_name: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub aircraft_name: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub entity_name: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub vessel_name: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub nickname: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub patronymic: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub matronymic: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub quality: Option<String>,
    #[sea_orm(column_type = "custom(\"TINYTEXT\")")]
    pub topmaj: String,
    #[sea_orm(ignore)]
    pub is_primary_215: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::name_sdn::Entity")]
    NameSdn,
}

impl Related<super::name_sdn::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::NameSdn.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
