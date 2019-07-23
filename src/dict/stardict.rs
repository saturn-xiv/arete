use std::path::Path;
use std::process::Command;

use serde::de::DeserializeOwned;

use super::super::errors::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct Dictionary {
    pub name: String,
    #[serde(rename(serialize = "wordCount", deserialize = "wordcount"))]
    pub word_count: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub dict: String,
    pub word: String,
    pub definition: String,
}
pub struct StarDict {
    data_dir: String,
}

impl StarDict {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        Ok(Self {
            data_dir: format!("{}", path.as_ref().display()),
        })
    }

    pub fn search(&self, word: &str) -> Result<Vec<Item>> {
        // FIXME command line inject
        self.execute(&format!("'{}'", word))
    }
    pub fn list(&self) -> Result<Vec<Dictionary>> {
        self.execute("-l")
    }

    fn execute<V: DeserializeOwned>(&self, arg: &str) -> Result<V> {
        let cmd = format!("sdcv -x -j --data-dir {} {}", self.data_dir, arg);
        info!("execute command: {}", cmd);
        let out = Command::new("sh").arg("-c").arg(cmd).output()?;
        if !out.status.success() {
            return Err(format_err!(
                "sdcv failed: {}",
                String::from_utf8(out.stderr)?
            ));
        }
        let it = serde_json::from_slice(&out.stdout)?;
        Ok(it)
    }
}
