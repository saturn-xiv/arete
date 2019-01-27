use std::ffi::OsStr;
use std::fs::{read_to_string, File};
use std::path::PathBuf;

use rocket_contrib::templates::Template;

use super::super::super::super::errors::Result;
use super::super::models::{self, Page};

#[get("/")]
pub fn index() -> Result<Template> {
    let pages = models::list()?;
    Ok(Template::render("wiki/index", json!({ "pages": pages })))
}

#[get("/<file..>")]
pub fn show(file: PathBuf) -> Result<Page> {
    let file = models::file(file);

    if Some(OsStr::new(models::MARKDOWN)) == file.extension() {
        let title = file.file_name();
        let body = read_to_string(&file)?;
        return Ok(Page::Html(Template::render(
            "wiki/show",
            json!({ "title": title, "body": body }),
        )));
    };
    Ok(Page::File(File::open(file)?))
}
