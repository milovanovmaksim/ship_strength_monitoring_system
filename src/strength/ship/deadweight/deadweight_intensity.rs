use log::{warn, debug};
use serde::Deserialize;
use crate::{strength::ship::{spatium_function::SpatiumFunction, ship_dimensions::ShipDimensions, load::shipload::Shipload}, core::json_file::JsonFile};

#[derive(Deserialize, Debug)]
pub struct DeadweightIntensity {
    shiploads: Shiploads
}


impl DeadweightIntensity {
    pub fn new(shiploads: Option<Vec<Shipload>>, ship_dimensions: ShipDimensions,) -> Self {
        DeadweightIntensity { shiploads, ship_dimensions }
    }

    ///
    /// Create the object from json file.
    pub fn from_json_file(file_path: String) -> Result<Self, String> {
        let json = JsonFile::new(file_path);
        match json.content() {
            Ok(content) => {
                match serde_json::from_reader(content) {
                    Ok(deadweight_intensity) => {
                        debug!("DeadweightIntensity::from_json_file | DeadweightIntensity has been created sucessfuly. {:?}", deadweight_intensity);
                        Ok(deadweight_intensity)
                    },
                    Err(err) => {
                        warn!("DeadweightIntensity::from_json_file | error: {:?}.",err);
                        return Err(err.to_string());
                    }
                }
            },
            Err(err) => {
                warn!("DeadweightIntensity::from_json_file | error: {:?}.",err);
                return Err(err);
            }
        }
    }

    pub fn deadweight_intensity(&self) -> Option<Vec<SpatiumFunction>> {
        match &self.shiploads {
            Some(loads) => {
                let deadweight_intensity = self.spatiums_filled_zero();
                for load in loads {
                    let load_intensity = load.load_intensity(&self.ship_dimensions);
                }
                Some(deadweight_intensity)
            },
            None => { None }
        }
    }
}