use std::ffi::OsStr;
use std::fs::{read_dir, DirEntry, File};
use std::io::{prelude::*, BufReader};
use std::path::{Path, PathBuf};

use super::super::super::errors::Result;

pub const MARKDOWN: &'static str = "md";

fn root() -> PathBuf {
    Path::new("tmp").join("wiki")
}

pub fn file<P: AsRef<Path>>(p: P) -> PathBuf {
    Path::new("tmp").join("wiki").join(p)
}

pub fn list() -> Result<Vec<(String, String)>> {
    let root = root();
    let mut items = Vec::new();
    let start = format!("{}", root.display()).len() + 1;

    visit_dirs(
        &root,
        &|it, ar| -> Result<()> {
            let path = it.path();
            let href = &format!("{}", path.display())[start..];
            if path.is_file() {
                if Some(OsStr::new(MARKDOWN)) == path.extension() {
                    let mut fd = BufReader::new(File::open(path)?);
                    let mut title = String::new();
                    fd.read_line(&mut title)?;
                    ar.push((href.to_string(), title));
                }
            }
            Ok(())
        },
        &mut items,
    )?;
    Ok(items)
}

fn visit_dirs(
    dir: &Path,
    cb: &Fn(&DirEntry, &mut Vec<(String, String)>) -> Result<()>,
    items: &mut Vec<(String, String)>,
) -> Result<()> {
    if dir.is_dir() {
        for entry in read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb, items)?;
            } else {
                cb(&entry, items)?;
            }
        }
    }
    Ok(())
}
