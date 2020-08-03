use std::ops::Deref;

use chrono::NaiveDateTime;
use juniper::{GraphQLInputObject, GraphQLObject};
use validator::Validate;

use super::super::super::super::{
    errors::Result,
    graphql::{context::Context, I16, ID},
};
use super::super::models::category::{Dao as CategoryDao, Item};

#[derive(GraphQLObject)]
pub struct Category {
    pub id: ID,
    pub parent_id: Option<ID>,
    pub name: String,
    pub icon: String,
    pub color: String,
    pub position: I16,
    pub updated_at: NaiveDateTime,
}

impl From<Item> for Category {
    fn from(it: Item) -> Self {
        Self {
            id: it.id.into(),
            parent_id: it.parent_id.map(|x| x.into()),
            name: it.name,
            icon: it.icon,
            color: it.color,
            position: it.position.into(),
            updated_at: it.updated_at,
        }
    }
}

impl Category {
    pub fn index(ctx: &Context) -> Result<Vec<Self>> {
        let db = ctx.db.deref();
        Ok(CategoryDao::all(db)?
            .into_iter()
            .map(|it| it.into())
            .collect::<_>())
    }

    pub fn show(ctx: &Context, id: ID) -> Result<Self> {
        let db = ctx.db.deref();
        Ok(CategoryDao::by_id(db, id.0)?.into())
    }
}

#[derive(GraphQLInputObject, Validate)]
pub struct Form {
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(length(min = 1))]
    pub icon: String,
    #[validate(length(min = 1))]
    pub color: String,
    pub parent_id: Option<ID>,
    pub position: I16,
}

impl Form {
    pub fn create(&self, ctx: &Context) -> Result<()> {
        self.validate()?;
        ctx.administrator()?;
        let db = ctx.db.deref();

        CategoryDao::create(
            db,
            self.parent_id.as_ref().map(|x| x.0),
            &self.name,
            &self.icon,
            &self.color,
            self.position.0,
        )?;

        Ok(())
    }
    pub fn update(&self, ctx: &Context, id: ID) -> Result<()> {
        self.validate()?;
        ctx.administrator()?;
        let db = ctx.db.deref();

        CategoryDao::update(
            db,
            id.0,
            self.parent_id.as_ref().map(|x| x.0),
            &self.name,
            &self.icon,
            &self.color,
            self.position.0,
        )?;
        Ok(())
    }
}

pub struct Destory;

impl Destory {
    pub fn execute(ctx: &Context, id: ID) -> Result<()> {
        ctx.administrator()?;
        let db = ctx.db.deref();
        CategoryDao::delete(db, id.0)?;
        Ok(())
    }
}
