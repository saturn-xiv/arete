// #[get("/")]
// pub fn index() -> TemplateResult {
//     let pages = models::list()?;
//     Ok(Template::render("wiki/index", json!({ "pages": pages })))
// }

// #[get("/<file..>")]
// pub fn show(file: PathBuf) -> StdResult<Page, HttpError> {
//     let file = models::file(file);

//     if Some(OsStr::new(models::MARKDOWN)) == file.extension() {
//         let title = file.file_name();
//         let body = read_to_string(&file)?;
//         return Ok(Page::Html(Template::render(
//             "wiki/show",
//             json!({ "title": title, "body": body }),
//         )));
//     };
//     Ok(Page::File(File::open(file)?))
// }
