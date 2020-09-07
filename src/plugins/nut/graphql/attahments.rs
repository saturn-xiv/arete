use std::ops::Deref;

use actix_multipart::Multipart;
use actix_web::{http::StatusCode, web, HttpResponse, Responder};
use chrono::NaiveDateTime;
use futures::StreamExt;
use juniper::GraphQLObject;
use uuid::Uuid;

use super::super::super::super::{
    errors::{Error, Result},
    graphql::{context::Context, I64, ID},
    orm::{Connection as Db, Pool as DbPool, ID as RID},
    storage::s3::Config as S3,
};
use super::super::{
    models::{
        attachment::{Dao as AttachmentDao, Item},
        policy::{Dao as PolicyDao, Role},
    },
    request::CurrentUser,
};

#[post("/attachments")]
pub async fn create(
    user: CurrentUser,
    db: web::Data<DbPool>,
    s3: web::Data<S3>,
    mut payload: Multipart,
) -> Result<impl Responder> {
    let db = db.get()?;
    let db = db.deref();
    let s3 = s3.deref();
    while let Some(item) = payload.next().await {
        let mut field = item.map_err(Error::Multipart)?;
        let ct = field.content_type().clone();
        if let Some(cd) = field.content_disposition() {
            if let Some(ref name) = cd.get_name() {
                match *name {
                    "file" => {
                        if let Some(name) = cd.get_filename() {
                            let mut body = Vec::new();
                            while let Some(chunk) = field.next().await {
                                let buf = chunk.map_err(Error::Multipart)?;
                                body.append(&mut buf.to_vec());
                            }
                            let bucket = user.0.nick_name.clone();
                            let key = Uuid::new_v4().to_string();
                            let size = body.len() as i64;
                            s3.put(&bucket, &key, body).await?;
                            let url = s3.get(&bucket, &key).await?;
                            AttachmentDao::create(db, user.0.id, name, &ct, &url, size)?;
                        }
                    }
                    _ => warn!("unknown form key {:?}", name),
                }
            }
        }
    }
    Ok(HttpResponse::Ok().json(()))
}

#[derive(GraphQLObject)]
pub struct Attachment {
    pub id: ID,
    pub title: String,
    pub size: I64,
    pub content_type: String,
    pub url: String,
    pub updated_at: NaiveDateTime,
}

impl From<Item> for Attachment {
    fn from(it: Item) -> Self {
        Self {
            id: it.id.into(),
            title: it.title,
            size: it.size.into(),
            content_type: it.content_type,
            url: it.url,
            updated_at: it.updated_at,
        }
    }
}

impl Attachment {
    pub fn index(ctx: &Context) -> Result<Vec<Self>> {
        let user = ctx.current_user()?;
        let db = ctx.db.deref();
        let items = if PolicyDao::is(db, user.id, &Role::Admin) {
            AttachmentDao::all(db)?
        } else {
            AttachmentDao::by_user(db, user.id)?
        };
        Ok(items.into_iter().map(|it| it.into()).collect::<_>())
    }

    pub fn can(db: &Db, user: RID, id: RID) -> Result<Item> {
        let it = AttachmentDao::by_id(db, id)?;
        if it.user_id == user || PolicyDao::is(db, user, &Role::Admin) {
            return Ok(it);
        }
        Err(Error::Http(StatusCode::FORBIDDEN).into())
    }
}

pub struct Destory;

impl Destory {
    pub fn execute(ctx: &Context, id: ID) -> Result<()> {
        let user = ctx.current_user()?;
        let db = ctx.db.deref();
        Attachment::can(db, user.id, id.0)?;
        AttachmentDao::delete(db, id.0)?;
        Ok(())
    }
}
