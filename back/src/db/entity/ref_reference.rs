use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "ref_reference")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "Text")]
    pub value: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::caatsa235::Entity")]
    Caatsa235,
    #[sea_orm(has_many = "super::eo13662dd::Entity")]
    Eo13662dd,
    #[sea_orm(has_many = "super::eo13846inf::Entity")]
    Eo13846inf,
    #[sea_orm(has_many = "super::eo14024dd::Entity")]
    Eo14024dd,
    #[sea_orm(has_many = "super::target::Entity")]
    Target,
}

impl Related<super::caatsa235::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Caatsa235.def()
    }
}

impl Related<super::eo13662dd::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Eo13662dd.def()
    }
}

impl Related<super::eo13846inf::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Eo13846inf.def()
    }
}

impl Related<super::eo14024dd::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Eo14024dd.def()
    }
}

impl Related<super::target::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Target.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
