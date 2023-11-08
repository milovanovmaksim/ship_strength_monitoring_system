use log::warn;
use serde::Deserialize;
use crate::{strength::ship::{load::shiploads::Shiploads, spatium_functions::SpatiumFunctions}, core::json_file::JsonFile};

#[derive(Deserialize, Debug)]
pub struct DeadweightIntensity {
    shiploads: Result<Shiploads, String>,
}


impl DeadweightIntensity {
    pub fn new(shiploads: Result<Shiploads, String>) -> Self {
        DeadweightIntensity { shiploads }
    }

    pub fn deadweight_intensity(&self) -> Result<SpatiumFunctions, String> {
        match &self.shiploads {
            Ok(shiploads) => {
                Ok(shiploads.intensity())
            }
            Err(err) => {
                warn!("DeadweightIntensity::deadweight_intensity | error: {:?}.",err);
                Err(err.to_string())
            }
        }
    }
}