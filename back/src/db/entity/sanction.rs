use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Model {
    pub id: i32,
    pub date: Date,
    pub status: String,
    pub topmaj: String,
    pub programs: Vec<super::program::Model>,
    pub sdn_id: i32,
}
