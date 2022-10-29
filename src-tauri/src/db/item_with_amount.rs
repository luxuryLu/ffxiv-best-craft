//! SeaORM Entity. Generated by sea-orm-codegen 0.9.2

use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "ItemWithAmount")]
pub struct Model {
    #[sea_orm(column_name = "Id", primary_key)]
    pub id: i32,
    #[sea_orm(column_name = "IngredientId")]
    pub ingredient_id: i32,
    #[sea_orm(column_name = "Amount")]
    pub amount: i32,
    #[sea_orm(column_name = "RecipeId")]
    pub recipe_id: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::recipes::Entity",
        from = "Column::RecipeId",
        to = "super::recipes::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Recipes,
    #[sea_orm(
        belongs_to = "super::items::Entity",
        from = "Column::IngredientId",
        to = "super::items::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Items,
}

impl Related<super::recipes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Recipes.def()
    }
}

impl Related<super::items::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Items.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}