use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    {
        let git_version = String::from_utf8(
            Command::new("git")
                .arg("describe")
                .arg("--tags")
                .arg("--always")
                .arg("--first-parent")
                .arg("--dirty")
                .output()
                .unwrap()
                .stdout,
        )
        .unwrap();
        let build_time =
            String::from_utf8(Command::new("date").arg("-u").output().unwrap().stdout).unwrap();

        let dest_path = Path::new(&out_dir).join("env.rs");
        let mut fd = File::create(&dest_path).unwrap();

        writeln!(fd, r#"pub const VERSION: &str = "{}";"#, git_version.trim()).unwrap();
        writeln!(
            fd,
            r#"pub const BUILD_TIME: &str = "{}";"#,
            build_time.trim()
        )
        .unwrap();
    }
}
