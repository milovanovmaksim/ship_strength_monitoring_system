use log::warn;
use serde::Deserialize;
use crate::strength::ship::{load::shiploads::Shiploads, ship_dimensions::ShipDimensions};

#[derive(Deserialize, Debug)]
pub struct DeadweightIntensity {
    shiploads: Shiploads,
    ship_dimensions: ShipDimensions
}


impl DeadweightIntensity {
    pub fn new(shiploads: Shiploads, ship_dimensions: ShipDimensions) -> Self {
        DeadweightIntensity { shiploads, ship_dimensions }
    }

    pub fn deadweight_intensity(&self) {
        todo!()
    }
}
