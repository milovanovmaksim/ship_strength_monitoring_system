use log::warn;
use serde::Deserialize;
use crate::{strength::ship::{load::{shiploads::Shiploads, load_sharing::LoadSharing}, spatium_functions::SpatiumFunctions, ship_dimensions::ShipDimensions}, core::json_file::JsonFile};

#[derive(Deserialize, Debug)]
pub struct DeadweightIntensity {
    load_sharing: LoadSharing,
    ship_dimensions: ShipDimensions
}


impl DeadweightIntensity {
    pub fn new(load_sharing: LoadSharing, ship_dimensions: ShipDimensions) -> Self {
        DeadweightIntensity { load_sharing, ship_dimensions }
    }

    pub fn deadweight_intensity(&self) {
        let number_spatiums = self.ship_dimensions.number_spatiums();
        let length_spatium = self.ship_dimensions.length_spatium();
        let length_between_perpendiculars = self.ship_dimensions.length_between_perpendiculars();
        let spatium_functions = SpatiumFunctions::filled_zeros(number_spatiums, length_spatium, length_between_perpendiculars);
        let shiploads = self.load_sharing.shared_loads();
        for shipload in shiploads.iter() {
            let spatium_functions = shipload.shipload_intensity(&self.ship_dimensions);

        }
    }



    
}