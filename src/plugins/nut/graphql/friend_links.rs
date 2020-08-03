use std::ops::Deref;

use chrono::NaiveDateTime;
use juniper::{GraphQLInputObject, GraphQLObject};
use validator::Validate;

use super::super::super::super::{
    errors::Result,
    graphql::{context::Context, I16, ID},
};
use super::super::models::friend_link::{Dao as FriendLinkDao, Item};

#[derive(GraphQLObject)]
pub struct FriendLink {
    pub id: ID,
    pub title: String,
    pub home: String,
    pub logo: String,
    pub position: I16,
    pub updated_at: NaiveDateTime,
}

impl From<Item> for FriendLink {
    fn from(it: Item) -> Self {
        Self {
            id: it.id.into(),
            title: it.title,
            home: it.home,
            logo: it.logo,
            position: it.position.into(),
            updated_at: it.updated_at,
        }
    }
}

impl FriendLink {
    pub fn index(ctx: &Context) -> Result<Vec<Self>> {
        let db = ctx.db.deref();
        Ok(FriendLinkDao::all(db)?
            .into_iter()
            .map(|it| it.into())
            .collect::<_>())
    }

    pub fn show(ctx: &Context, id: ID) -> Result<Self> {
        let db = ctx.db.deref();
        Ok(FriendLinkDao::by_id(db, id.0)?.into())
    }
}

#[derive(GraphQLInputObject, Validate)]
pub struct Form {
    #[validate(length(min = 1))]
    pub title: String,
    #[validate(length(min = 1))]
    pub home: String,
    #[validate(length(min = 1))]
    pub logo: String,
    pub position: I16,
}

impl Form {
    pub fn create(&self, ctx: &Context) -> Result<()> {
        self.validate()?;
        ctx.administrator()?;
        let db = ctx.db.deref();
        FriendLinkDao::create(db, &self.title, &self.home, &self.logo, self.position.0)?;
        Ok(())
    }
    pub fn update(&self, ctx: &Context, id: ID) -> Result<()> {
        self.validate()?;
        ctx.administrator()?;
        let db = ctx.db.deref();
        FriendLinkDao::update(
            db,
            id.0,
            &self.title,
            &self.home,
            &self.logo,
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
        FriendLinkDao::delete(db, id.0)?;
        Ok(())
    }
}
