use log::{debug, warn};
use serde::Deserialize;

use crate::core::json_file::JsonFile;

///
/// Масса судна, когда оно было построено на верфи.
///Не включает массу любого расходного материала, такого как топливо, вода, масло или другие расходные материалы.
#[derive(Deserialize, Debug, Clone, Copy)]
pub struct Lightweight {
    lightweight: f64,
}

impl Lightweight {
    pub fn new(lightweight: f64) -> Self {
        Lightweight { lightweight }
    }

    /// Create the object from json file.
    pub fn from_json_file(file_path: String) -> Result<Self, String> {
        let json = JsonFile::new(file_path);
        match json.content() {
            Ok(content) => match serde_json::from_reader(content) {
                Ok(lightweight) => {
                    debug!("Lightweight::from_json_file | Lightweight has been created sucessfuly. {:?}", lightweight);
                    Ok(lightweight)
                }
                Err(err) => {
                    warn!("Lightweight::from_json_file | error: {:?}.", err);
                    return Err(err.to_string());
                }
            },
            Err(err) => {
                warn!("Lightweight::from_json_file | error: {:?}.", err);
                return Err(err);
            }
        }
    }

    pub fn lightweight(&self) -> f64 {
        self.lightweight
    }
}
