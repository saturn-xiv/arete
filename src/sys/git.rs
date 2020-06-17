use std::path::Path;

use git2::Repository;

use super::super::errors::Result;

pub fn upgrade<P: AsRef<Path>>(url: &str, path: P) -> Result<()> {
    let path = path.as_ref();
    let repo = if path.exists() {
        Repository::open(path)?
    } else {
        Repository::clone(url, path)?
    };
    repo.find_remote("origin")?.fetch(&["master"], None, None)?;
    Ok(())
}
