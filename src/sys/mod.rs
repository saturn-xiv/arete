pub mod git;
pub mod image_magick;
pub mod network;
pub mod nmap;
pub mod timezone;
pub mod vpn;
pub mod watchdog;

use std::net::ToSocketAddrs;
use std::process;

use chrono::{DateTime, Local, TimeZone};
use failure::SyncFailure;
use ntp::unix_time::Instant;

use super::errors::Result;

pub fn ntp<T: ToSocketAddrs>(url: T) -> Result<DateTime<Local>> {
    let response = ntp::request(url).map_err(SyncFailure::new)?;
    debug!("receive time {:?}", response);
    let unix_time = Instant::from(response.transmit_timestamp);
    let local_time = Local.timestamp(unix_time.secs(), unix_time.subsec_nanos() as _);
    Ok(local_time)
}

pub fn reboot() -> Result<()> {
    warn!("reboot system!!!");
    nix::unistd::sync();
    nix::sys::reboot::reboot(nix::sys::reboot::RebootMode::RB_AUTOBOOT)?;
    Ok(())
}

pub fn hostname() -> Result<String> {
    let mut buf = [0u8; 64];
    let it = nix::unistd::gethostname(&mut buf)?.to_str()?;
    Ok(it.to_string())
}

pub fn uts_name() -> nix::sys::utsname::UtsName {
    nix::sys::utsname::uname()
}

pub fn sys_info() -> Result<nix::sys::sysinfo::SysInfo> {
    let it = nix::sys::sysinfo::sysinfo()?;
    Ok(it)
}

pub fn pid() -> u32 {
    process::id()
}
