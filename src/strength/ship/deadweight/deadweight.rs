use log::{debug, error};
use serde::Deserialize;
use crate::{core::json_file::JsonFile, strength::ship::load::shiploads::Shiploads};



#[derive(Deserialize, Debug)]
pub struct Deadweight {
    shiploads: Shiploads,

}


impl Deadweight {
    pub fn new(shiploads: Shiploads) -> Self {
        Deadweight { shiploads }
    }

    ///
    /// Create the object from json file.
    pub fn from_json_file(file_path: String) -> Result<Self, String> {
        let json = JsonFile::new(file_path);
        match json.content() {
            Ok(content) => {
                match serde_json::from_reader(content) {
                    Ok(deadweight) => {
                        debug!("Deadweight::from_json_file | Deadweight has been created sucessfuly. {:?}", deadweight);
                        Ok(deadweight)
                    },
                    Err(err) => {
                        error!("Deadweight::from_json_file | error: {:?}.",err);
                        return Err(err.to_string());
                    }
                }
            },
            Err(err) => {
                error!("Deadweight::from_json_file | error: {:?}.",err);
                return Err(err);
            }
        }
    }

    pub fn deadweight(&self) -> f64 {
        self.shiploads.sum()
    }
}
