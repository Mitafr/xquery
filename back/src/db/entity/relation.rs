use sea_orm::{entity::prelude::*, EntityTrait, RelationTrait};

#[derive(Clone, Debug, DeriveEntityModel, Default)]
#[sea_orm(table_name = "relation")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i32,
    pub linked_to: i32,
    pub relation_type_id: i32,
    #[sea_orm(ignore)]
    pub from_profile_id: i32,
    #[sea_orm(column_type = "custom(\"TINYTEXT\")")]
    pub topmaj: String,
}

impl PartialEq for Model {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.linked_to == other.linked_to
            && self.relation_type_id == other.relation_type_id
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::sdn::Entity",
        from = "Column::LinkedTo",
        to = "super::sdn::Column::FixedRef",
        on_update = "Restrict",
        on_delete = "Restrict"
    )]
    Sdn,
    #[sea_orm(has_many = "super::relation_sdn::Entity")]
    RelationSdn,
}

impl Related<super::sdn::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Sdn.def()
    }
}

impl Related<super::relation_sdn::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RelationSdn.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
