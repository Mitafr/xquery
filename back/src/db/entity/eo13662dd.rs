use sea_orm::{entity::prelude::*, EntityTrait, RelationTrait};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Default)]
#[sea_orm(table_name = "eo13662dd")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i32,
    pub reference_id: Option<i32>,
    #[sea_orm(column_type = "custom(\"TINYTEXT\")")]
    pub topmaj: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::ref_reference::Entity",
        from = "Column::ReferenceId",
        to = "super::ref_reference::Column::Id",
        on_update = "Restrict",
        on_delete = "Restrict"
    )]
    RefReference,
    #[sea_orm(has_many = "super::eo13662dd_sdn::Entity")]
    Eo13662ddSdn,
}

impl Related<super::ref_reference::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RefReference.def()
    }
}

impl Related<super::eo13662dd_sdn::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Eo13662ddSdn.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
