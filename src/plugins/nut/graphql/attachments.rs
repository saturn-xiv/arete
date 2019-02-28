use std::ops::Deref;

use chrono::NaiveDateTime;
use hyper::StatusCode;
use validator::Validate;
// use multipart::server::{
//     save::{Entries, SaveResult::*},
//     Multipart,
// };

use super::super::super::super::{
    errors::{Error, Result},
    graphql::{context::Context, session::Session, Handler, I64},
};
use super::super::models::{
    attachment::{Dao as AttachmentDao, Item},
    policy::{Dao as PolicyDao, Role},
};

#[derive(GraphQLObject)]
pub struct Attachment {
    pub id: I64,
    pub title: String,
    pub mime_type: String,
    pub url: String,
    pub size: I64,
    pub updated_at: NaiveDateTime,
}

impl From<Item> for Attachment {
    fn from(it: Item) -> Self {
        Self {
            id: I64(it.id),
            title: it.title,
            mime_type: it.mime_type,
            url: it.url,
            size: I64(it.size),
            updated_at: it.updated_at,
        }
    }
}

#[derive(Validate)]
pub struct Show {
    pub id: i64,
}

impl Handler for Show {
    type Item = Attachment;
    fn handle(&self, c: &Context, _s: &Session) -> Result<Self::Item> {
        let db = c.db()?;
        let db = db.deref();
        let it = AttachmentDao::by_id(db, &self.id)?;
        Ok(it.into())
    }
}

#[derive(Validate)]
pub struct Index {}

impl Handler for Index {
    type Item = Vec<Attachment>;
    fn handle(&self, c: &Context, s: &Session) -> Result<Self::Item> {
        let db = c.db()?;
        let db = db.deref();
        let user = s.current_user()?;
        let items = if PolicyDao::can(db, &user.id, &Role::Admin, &None) {
            AttachmentDao::all(db)?
        } else {
            AttachmentDao::by_user(db, &user.id)?
        };

        Ok(items.into_iter().map(|x| x.into()).collect())
    }
}

#[derive(Validate)]
pub struct Destroy {
    pub id: i64,
}

impl Handler for Destroy {
    type Item = ();
    fn handle(&self, c: &Context, s: &Session) -> Result<Self::Item> {
        let db = c.db()?;
        let db = db.deref();
        let user = s.current_user()?;
        let it = AttachmentDao::by_id(db, &self.id)?;
        if it.user_id == self.id || PolicyDao::can(db, &user.id, &Role::Admin, &None) {
            AttachmentDao::delete(db, &self.id)?;
            return Ok(());
        }
        Err(Error::Http(StatusCode::FORBIDDEN).into())
    }
}

// #[post("/attachments", data = "<data>")]
// pub fn create(
//     content_type: &ContentType,
//     user: CurrentUser,
//     data: Data,
//     i18n: I18n,
//     cfg: State<Config>,
//     db: Database,
// ) -> JsonResult<Vec<Attachment>> {
//     let db = db.deref();
//     let cfg = cfg.deref();
//     if content_type.is_form_data() {
//         if let Some((_, title)) = content_type.params().find(|&(k, _)| k == "boundary") {
//             let mut items = Vec::new();
//             for (title, mime_type, url, size) in process_upload(title, data, cfg)? {
//                 items.push(AttachmentDao::create(
//                     db,
//                     &user.id,
//                     &title,
//                     &mime_type,
//                     &url,
//                     &(size as i64),
//                 )?);
//             }
//             return Ok(Json(items));
//         }
//     }
//     Err(i18n.e("flashes.bad-action", &None::<String>).into())
// }

// impl Upload {
//     pub fn upload(&self, boundary: &str, data: Data) -> Result<Vec<UploadItem>> {
//         match Multipart::with_body(data.open(), boundary).save().temp() {
//             Full(entries) => self.entries(entries),
//             Partial(partial, _) => self.entries(partial.entries),
//             Error(e) => Err(e.into()),
//         }
//     }
//     fn entries(&self, entries: Entries) -> Result<Vec<UploadItem>> {
//         let now = Utc::now().format("%F").to_string();
//         let mut items = Vec::new();
//         for (_, files) in entries.fields {
//             for src in files {
//                 let title = src.headers.filename.ok_or(format_err!("empty filename"))?;
//                 let mime_type = src
//                     .headers
//                     .content_type
//                     .ok_or(format_err!("empty content type"))?
//                     .to_string();
//                 let size = src.data.size();

//                 let mut file = Path::new(&now).join(Uuid::new_v4().to_string());
//                 if let Some(ext) = Path::new(&title).extension() {
//                     file.set_extension(ext);
//                 }

//                 match self {
//                     Upload::Filesystem(ref root) => {
//                         let file = Path::new(root).join(&file);
//                         if let Some(d) = file.parent() {
//                             create_dir_all(d)?;
//                         }
//                         let mut buf = Vec::new();
//                         src.data.readable()?.read_to_end(&mut buf)?;
//                         let mut dst = File::create(file)?;
//                         dst.write_all(&buf)?;
//                     }
//                 }

//                 items.push((
//                     title,
//                     mime_type,
//                     Path::new("upload").join(&file).display().to_string(),
//                     size,
//                 ));
//             }
//         }

//         Ok(items)
//     }
// }
