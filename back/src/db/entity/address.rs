use sea_orm::{entity::prelude::*, EntityTrait, RelationTrait};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Default)]
#[sea_orm(table_name = "address")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i32,
    #[sea_orm(column_type = "Text", nullable)]
    pub address: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub city: Option<String>,
    pub country: Option<i32>,
    #[sea_orm(column_type = "Text", nullable)]
    pub postal_code: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub region: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub state: Option<String>,
    pub is_primary: bool,
    #[sea_orm(column_type = "Text")]
    pub topmaj: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::ref_country::Entity",
        from = "Column::Country",
        to = "super::ref_country::Column::Id",
        on_update = "Restrict",
        on_delete = "Restrict"
    )]
    RefCountry,
    #[sea_orm(has_many = "super::address_sdn::Entity")]
    AddressSdn,
}

impl Related<super::ref_country::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RefCountry.def()
    }
}

impl Related<super::address_sdn::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AddressSdn.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
