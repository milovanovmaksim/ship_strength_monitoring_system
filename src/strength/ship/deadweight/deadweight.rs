use log::{debug, warn};
use serde::Deserialize;
use crate::{core::json_file::JsonFile, strength::ship::loads::load::load::Load};



#[derive(Deserialize, Debug)]
pub struct Deadweight {
    loads: Option<Vec<Load>>,

}


impl Deadweight {
    pub fn new(loads: Option<Vec<Load>>) -> Self {
        Deadweight { loads }
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
                        warn!("Deadweight::from_json_file | error: {:?}.",err);
                        return Err(err.to_string());
                    }
                }
            },
            Err(err) => {
                warn!("Deadweight::from_json_file | error: {:?}.",err);
                return Err(err);
            }
        }
    }

    pub fn deadweight(&self) -> f64 {
        match &self.loads {
            Some(loads) => {
                let mut deadweight = 0.0;
                for load in loads {
                    deadweight += load.value();
                }
                deadweight
            }
            None => { 0.0 }
        }
    }
}
