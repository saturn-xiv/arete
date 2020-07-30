use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Component, Path};
use std::thread;
use std::time::Duration;

use super::super::errors::Result;

pub fn feed() -> Result<()> {
    let wd = Path::new(&Component::RootDir).join("dev").join("watchdog");
    info!("start watchdog thread({})", wd.display());
    let mut fd = OpenOptions::new()
        .read(false)
        .write(true)
        .create(false)
        .append(true)
        .open(wd)?;
    thread::spawn(move || loop {
        log::trace!("feed watchdog");
        if let Err(e) = write!(&mut fd, "1") {
            error!("{:?}", e);
        }
        thread::sleep(Duration::from_secs(5));
    });
    Ok(())
}
