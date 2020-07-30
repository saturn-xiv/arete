use std::collections::HashMap;
use std::fmt::Debug;
use std::path::Path;
use std::time::Duration;

use actix_web::{HttpResponse, Responder};
use chrono::{NaiveDateTime, Utc};
use handlebars::Handlebars;
use mime::TEXT_HTML_UTF_8;
use serde::ser::Serialize;

use super::{
    cache::{Connection as Cache, Provider as CacheProvider},
    crypto::Secret,
    errors::Result,
    i18n::locale::Dao as LocaleDao,
    orm::Connection as Db,
    parser::from_toml,
    plugins::nut::models::link::Dao as LinkDao,
    settings::Dao as SettingDao,
};

pub fn render<M: Serialize + Debug, E: Secret>(
    id: &str,
    lang: &str,
    db: &Db,
    sec: &E,
    ch: &mut Cache,
    hbs: &Handlebars<'_>,
    tpl: &str,
    model: &M,
) -> Result<impl Responder> {
    let theme = match SettingDao::get(db, sec, &"site.theme".to_string()) {
        Ok(it) => it,
        Err(_) => "bootstrap".to_string(),
    };

    let body = ch.get(
        &format!("{}://{}", lang, id),
        Duration::from_secs(60 * 60 * 24),
        &|| {
            let body = hbs.render(
                &format!("{}/views/{}", theme, tpl),
                &Layout {
                    page: model,
                    site: Site::new(db, sec, lang)?,
                },
            )?;
            Ok(body)
        },
    )?;

    Ok(HttpResponse::Ok()
        .content_type(TEXT_HTML_UTF_8.to_string())
        .body(body))
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Layout<T: Serialize + Debug> {
    pub page: T,
    pub site: Site,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Author {
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Site {
    pub title: String,
    pub subhead: String,
    pub author: Author,
    pub keywords: Vec<String>,
    pub description: String,
    pub copyright: String,
    pub lang: String,
    pub languages: Vec<String>,
    pub locales: HashMap<String, String>,
    pub nav: HashMap<String, Nav>,
    pub created_at: NaiveDateTime,
}

impl Site {
    pub fn new<S: Secret>(db: &Db, sec: &S, lang: &str) -> Result<Self> {
        let fd = Path::new("tmp")
            .join("site")
            .join(lang)
            .with_extension("toml");
        if fd.exists() {
            return from_toml(&fd);
        }

        let locales = {
            let mut items = HashMap::new();
            for it in LocaleDao::by_lang(db, lang)? {
                items.insert(it.code, it.message);
            }
            items
        };

        let nav = {
            let mut items = HashMap::new();
            for loc in LinkDao::loc_by_lang(db, lang)?.iter() {
                if let Ok(root) = LinkDao::by_lang_loc_y(db, lang, loc, 0) {
                    for it in root {
                        let mut nav = Nav {
                            title: it.label.clone(),
                            href: it.href.clone(),
                            children: Vec::new(),
                        };
                        if let Ok(children) = LinkDao::by_lang_loc_x(db, lang, loc, it.x) {
                            for it in children {
                                nav.children.push(Link {
                                    title: it.label.clone(),
                                    href: it.href.clone(),
                                });
                            }
                        }
                        items.insert(loc.to_string(), nav);
                    }
                }
            }
            items
        };

        let it = Self {
            title: LocaleDao::by_lang_and_code(db, lang, "site.title")?.message,
            subhead: LocaleDao::by_lang_and_code(db, lang, "site.subhead")?.message,
            keywords: SettingDao::get(db, sec, &"site.keywords".to_string())?,
            author: SettingDao::get(db, sec, &"site.author".to_string())?,
            description: LocaleDao::by_lang_and_code(db, lang, "site.description")?.message,
            copyright: SettingDao::get(db, sec, &"site.copyright".to_string())?,
            lang: lang.to_string(),
            languages: LocaleDao::languages(db)?,
            locales,
            nav,
            created_at: Utc::now().naive_local(),
        };
        Ok(it)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Nav {
    pub title: String,
    pub href: String,
    pub children: Vec<Link>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Link {
    pub title: String,
    pub href: String,
}
