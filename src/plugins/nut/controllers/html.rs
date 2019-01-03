use std::path::PathBuf;

use actix_web::{fs::NamedFile, HttpRequest, Result};

use super::super::State;

pub fn third(req: &HttpRequest<State>) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("file")?;
    Ok(NamedFile::open(req.state().config.http.third().join(path))?)
}

pub fn assets(req: &HttpRequest<State>) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("file")?;
    Ok(NamedFile::open(
        req.state().config.http.assets().join(path),
    )?)
}

pub fn global(req: &HttpRequest<State>) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("file")?;
    Ok(NamedFile::open(
        req.state().config.http.global().join(path),
    )?)
}

pub fn attachments(req: &HttpRequest<State>) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("file")?;
    Ok(NamedFile::open(
        req.state().config.http.upload().join(path),
    )?)
}
