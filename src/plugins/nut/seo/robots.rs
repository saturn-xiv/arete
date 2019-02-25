// http://www.robotstxt.org/
// "/robots.txt"
pub fn txt() -> &'static str {
    // TODO
    r#"
User-agent: *
Disallow: /api/
"#
}
