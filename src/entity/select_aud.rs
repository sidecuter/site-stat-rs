//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.1

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "select_aud")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub user_id: Uuid,
    pub visit_date: DateTime,
    pub auditory_id: String,
    pub success: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::aud::Entity",
        from = "Column::AuditoryId",
        to = "super::aud::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Aud,
    #[sea_orm(
        belongs_to = "super::user_id::Entity",
        from = "Column::UserId",
        to = "super::user_id::Column::UserId",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    UserId,
}

impl Related<super::aud::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Aud.def()
    }
}

impl Related<super::user_id::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserId.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
