use sea_orm::{entity::prelude::*, EntityTrait, RelationTrait};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Default)]
#[sea_orm(table_name = "equity_ticker")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "Text", nullable)]
    pub equity_ticker: String,
    #[sea_orm(column_type = "custom(\"TINYTEXT\")")]
    pub topmaj: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::equity_ticker_sdn::Entity")]
    EquityTickerSdn,
}

impl Related<super::equity_ticker_sdn::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EquityTickerSdn.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
