use std::ops::Deref;

use rocket_contrib::json::Json;
use validator::Validate;

use super::super::super::super::super::super::super::{
    errors::Result, i18n::locale::Dao as LocaleDao, orm::Database,
};
use super::super::super::super::super::request::Administrator;

#[derive(Debug, Validate, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Form {
    #[validate(length(min = "1"))]
    pub title: String,
    #[validate(length(min = "1"))]
    pub subhead: String,
    #[validate(length(min = "1"))]
    pub description: String,
    #[validate(length(min = "1"))]
    pub keywords: String,
    #[validate(length(min = "1"))]
    pub copyright: String,
}

const KEY: &'static str = "site.";

#[get("/admin/site/info/<lang>")]
pub fn get(_user: Administrator, lang: String, db: Database) -> Result<Json<Form>> {
    let db = db.deref();

    let it = Form {
        title: LocaleDao::by_lang_and_code(db, &lang, &(KEY.to_owned() + "title"))?.message,
        subhead: LocaleDao::by_lang_and_code(db, &lang, &(KEY.to_owned() + "subhead"))?.message,
        description: LocaleDao::by_lang_and_code(db, &lang, &(KEY.to_owned() + "description"))?
            .message,
        keywords: LocaleDao::by_lang_and_code(db, &lang, &(KEY.to_owned() + "keywords"))?.message,
        copyright: LocaleDao::by_lang_and_code(db, &lang, &(KEY.to_owned() + "copyright"))?.message,
    };
    Ok(Json(it))
}

#[post("/admin/site/info/<lang>", format = "json", data = "<form>")]
pub fn post(
    _user: Administrator,
    lang: String,
    db: Database,
    form: Json<Form>,
) -> Result<Json<()>> {
    form.validate()?;
    let db = db.deref();
    let form = form.deref();

    for (k, v) in vec![
        (KEY.to_owned() + "title", &form.title),
        (KEY.to_owned() + "subhead", &form.subhead),
        (KEY.to_owned() + "keywords", &form.keywords),
        (KEY.to_owned() + "description", &form.description),
        (KEY.to_owned() + "copyright", &form.copyright),
    ] {
        let it = LocaleDao::by_lang_and_code(db, &lang, &k)?;
        LocaleDao::update(db, &it.id, &lang, &k, v)?;
    }

    Ok(Json(()))
}
