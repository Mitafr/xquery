use sea_orm::{entity::prelude::*, EntityTrait, RelationTrait};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Default)]
#[sea_orm(table_name = "document")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i32,
    pub doctype: Option<i32>,
    #[sea_orm(column_type = "Text", nullable)]
    pub registration_number: Option<String>,
    pub issued_by: Option<i32>,
    pub issued_date: Option<Date>,
    pub expiration_date: Option<Date>,
    #[sea_orm(column_type = "custom(\"TINYTEXT\")")]
    pub topmaj: String,
    #[sea_orm(ignore)]
    pub identity: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::ref_country::Entity",
        from = "Column::IssuedBy",
        to = "super::ref_country::Column::Id",
        on_update = "Restrict",
        on_delete = "Restrict"
    )]
    RefCountry,
    #[sea_orm(
        belongs_to = "super::ref_document::Entity",
        from = "Column::Doctype",
        to = "super::ref_document::Column::Id",
        on_update = "Restrict",
        on_delete = "Restrict"
    )]
    RefDocument,
    #[sea_orm(has_many = "super::document_identity::Entity")]
    DocumentIdentity,
}

impl Related<super::ref_country::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RefCountry.def()
    }
}

impl Related<super::ref_document::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RefDocument.def()
    }
}

impl Related<super::document_identity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DocumentIdentity.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
