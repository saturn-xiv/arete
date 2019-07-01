pub mod git;
pub mod image_magick;
pub mod network;
pub mod nmap;
pub mod timezone;
pub mod vpn;

use std::process;

use nix;

use super::errors::Result;

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
