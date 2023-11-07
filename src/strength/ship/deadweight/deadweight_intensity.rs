use log::{warn, debug};
use serde::Deserialize;
use crate::{strength::ship::{load::shiploads::Shiploads, spatium_functions::SpatiumFunctions}, core::json_file::JsonFile};

#[derive(Deserialize, Debug)]
pub struct DeadweightIntensity {
    shiploads: Shiploads,
}


impl DeadweightIntensity {
    pub fn new(shiploads: Shiploads) -> Self {
        DeadweightIntensity { shiploads }
    }

    pub fn deadweight_intensity(&self) -> SpatiumFunctions {
        self.shiploads.intensity()
    }
}