use serde::Deserialize;
use crate::strength::ship::{load::{shiploads::Shiploads, shipload_intensity::ShiploadIntensity}, ship_dimensions::ShipDimensions, spatium_functions::SpatiumFunctions};

#[derive(Deserialize, Debug)]
pub struct DeadweightIntensity {
    shiploads: Shiploads
}


impl DeadweightIntensity {
    pub fn new(shiploads: Shiploads) -> Self {
        DeadweightIntensity { shiploads }
    }

    pub fn spatium_functions(&self, ship_dimensions: &ShipDimensions) -> SpatiumFunctions {
        let number_spatiums = ship_dimensions.number_spatiums();
        let length_between_perpendiculars = ship_dimensions.length_between_perpendiculars();
        let mut spatium_functions = SpatiumFunctions::filled_zeros(number_spatiums, length_between_perpendiculars);
        for shipload in self.shiploads.as_ref() {
            let shipload_intensity = ShiploadIntensity::new(shipload);
            for spatium_function in shipload_intensity.spatium_functions(ship_dimensions).as_ref() {
                spatium_functions.add_spatium_function(spatium_function)
            }
        }
        spatium_functions
    }
}
