// http://www.robotstxt.org/
#[get("/robots.txt")]
pub fn txt() -> &'static str {
    // TODO
    r#"
User-agent: *
Disallow: /api/
"#
}
