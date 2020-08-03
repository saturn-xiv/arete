use std::ops::Deref;

use chrono::NaiveDateTime;
use juniper::{GraphQLInputObject, GraphQLObject};
use validator::Validate;

use super::super::super::super::{
    errors::Result,
    graphql::{context::Context, I16, ID},
};
use super::super::models::card::{Dao as CardDao, Item};

#[derive(GraphQLObject)]
pub struct Card {
    pub id: ID,
    pub title: String,
    pub body: String,
    pub media_type: String,
    pub action: String,
    pub href: String,
    pub logo: String,
    pub loc: String,
    pub position: I16,
    pub updated_at: NaiveDateTime,
}

impl From<Item> for Card {
    fn from(it: Item) -> Self {
        Self {
            id: it.id.into(),
            title: it.title,
            body: it.body,
            media_type: it.media_type,
            action: it.action,
            href: it.href,
            logo: it.logo,
            loc: it.loc,
            position: it.position.into(),
            updated_at: it.updated_at,
        }
    }
}

impl Card {
    pub fn index(ctx: &Context) -> Result<Vec<Self>> {
        let db = ctx.db.deref();
        Ok(CardDao::by_lang(db, &ctx.locale)?
            .into_iter()
            .map(|it| it.into())
            .collect::<_>())
    }

    pub fn show(ctx: &Context, id: ID) -> Result<Self> {
        let db = ctx.db.deref();
        Ok(CardDao::by_id(db, id.0)?.into())
    }
}

#[derive(GraphQLInputObject, Validate)]
pub struct Form {
    #[validate(length(min = 1))]
    pub title: String,
    #[validate(length(min = 1))]
    pub body: String,
    #[validate(length(min = 1))]
    pub media_type: String,
    #[validate(length(min = 1))]
    pub action: String,
    #[validate(length(min = 1))]
    pub href: String,
    #[validate(length(min = 1))]
    pub logo: String,
    #[validate(length(min = 1))]
    pub loc: String,
    pub position: I16,
}

impl Form {
    pub fn create(&self, ctx: &Context) -> Result<()> {
        self.validate()?;
        ctx.administrator()?;
        let db = ctx.db.deref();

        CardDao::create(
            db,
            &ctx.locale,
            &self.title,
            &self.logo,
            &self.body,
            &self.media_type.parse()?,
            &self.href,
            &self.action,
            &self.loc,
            self.position.0,
        )?;
        Ok(())
    }
    pub fn update(&self, ctx: &Context, id: ID) -> Result<()> {
        self.validate()?;
        ctx.administrator()?;
        let db = ctx.db.deref();

        CardDao::update(
            db,
            id.0,
            &self.title,
            &self.logo,
            &self.body,
            &self.media_type.parse()?,
            &self.href,
            &self.action,
            &self.loc,
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
        CardDao::delete(db, id.0)?;
        Ok(())
    }
}
