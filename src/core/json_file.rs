use std::{fs::File, io::BufReader};

use log::warn;
use serde::Deserialize;

///
/// Reads json file
pub struct JsonFile {
    file_path: String,
}

impl JsonFile {
    pub fn new(file_path: String) -> Self {
        JsonFile { file_path }
    }

    ///
    /// Return content json file.
    pub fn content(&self) -> Result<BufReader<File>, String>
    {
        match File::open(&self.file_path) {
            Ok(file) => {
                Ok(BufReader::new(file))

            },
            Err(err) => {
                warn!("JsonFile.content() | error {:?}", err);
                return Err(err.to_string());
            }
        }
    }
}
