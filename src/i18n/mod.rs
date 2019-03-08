// pub mod graphql;
pub mod locale;
pub mod schema;

use failure::Error;
use mustache;
use serde::ser::Serialize;

use super::orm::Connection;

use self::locale::Dao;

pub trait I18n {
    fn exist(&self, lang: &String) -> bool;
    fn tr<S: Serialize>(&self, lang: &String, code: &String, args: &Option<S>) -> Option<String>;
    fn e<C: Into<String>, S: Serialize>(&self, lang: &String, code: C, args: &Option<S>) -> Error;
    fn t<C: Into<String>, S: Serialize>(&self, lang: &String, code: C, args: &Option<S>) -> String;
}

impl I18n for Connection {
    fn exist(&self, lang: &String) -> bool {
        if let Ok(items) = Dao::languages(self) {
            return items.contains(lang);
        }
        false
    }

    fn tr<S: Serialize>(&self, lang: &String, code: &String, args: &Option<S>) -> Option<String> {
        if let Ok(it) = Dao::by_lang_and_code(self, lang, code) {
            if let Ok(tpl) = mustache::compile_str(&it.message) {
                if let Ok(msg) = tpl.render_to_string(args) {
                    return Some(msg);
                }
            }
        }
        None
    }

    fn e<C: Into<String>, S: Serialize>(&self, lang: &String, code: C, args: &Option<S>) -> Error {
        let msg = self.t(lang, code, args);
        format_err!("{}", msg)
    }

    fn t<C: Into<String>, S: Serialize>(&self, lang: &String, code: C, args: &Option<S>) -> String {
        let code = code.into();
        match self.tr(lang, &code, args) {
            Some(msg) => msg,
            None => format!("{}.{}", lang, code),
        }
    }
}
