// https://validator.w3.org/feed/docs/atom.html
#[get("/rss/<lang>")]
pub fn atom(lang: String) -> String {
    // TODO
    lang
}
