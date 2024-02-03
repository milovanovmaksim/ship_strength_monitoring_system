use serde::Deserialize;
use crate::strength::ship::{load::{shiploads::Shiploads, shipload_intensity::ShiploadIntensity}, ship_dimensions::ShipDimensions, spatium_functions::SpatiumFunctions};

#[derive(Deserialize, Debug)]
pub struct DeadweightIntensity {
    shiploads: Shiploads,
    ship_dimensions: ShipDimensions,
}


impl DeadweightIntensity {
    pub fn new(shiploads: Shiploads, ship_dimensions: ShipDimensions,) -> Self {
        DeadweightIntensity { shiploads, ship_dimensions }
    }

    pub fn spatium_functions(&self) -> SpatiumFunctions {
        let number_spatiums = self.ship_dimensions.number_spatiums();
        let length_between_perpendiculars = self.ship_dimensions.length_between_perpendiculars();
        let mut spatium_functions = SpatiumFunctions::filled_zeros(number_spatiums, length_between_perpendiculars);
        let deadweight = self.shiploads.sum();
        for shipload in self.shiploads.as_ref() {
            let shipload_intensity = ShiploadIntensity::new(shipload, &self.ship_dimensions, deadweight);
            for spatium_function in shipload_intensity.shipload_intensity().as_ref() {
                spatium_functions.add_spatium_function(spatium_function)
            }
        }
        spatium_functions
    }
}
