use log::warn;
use serde::Deserialize;
use crate::{strength::ship::{load::{shiploads::Shiploads, load_sharing::LoadSharing}, spatium_functions::SpatiumFunctions}, core::json_file::JsonFile};

#[derive(Deserialize, Debug)]
pub struct DeadweightIntensity {
    load_sharing: LoadSharing,
}


impl DeadweightIntensity {
    pub fn new(load_sharing: LoadSharing,) -> Self {
        DeadweightIntensity { load_sharing }
    }

    pub fn deadweight_intensity(&self) {
        let shiploads = self.load_sharing.shared_loads();
        for shipload in shiploads.iter() {
            

        }
    }
}