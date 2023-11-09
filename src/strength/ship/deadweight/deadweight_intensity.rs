use serde::Deserialize;
use crate::strength::ship::{load::shiploads::Shiploads, ship_dimensions::ShipDimensions, spatium_functions::SpatiumFunctions};

#[derive(Deserialize, Debug)]
pub struct DeadweightIntensity {
    shiploads: Shiploads,
    ship_dimensions: ShipDimensions,
}


impl DeadweightIntensity {
    pub fn new(shiploads: Shiploads, ship_dimensions: ShipDimensions,) -> Self {
        DeadweightIntensity { shiploads, ship_dimensions }
    }

    pub fn deadweight_intensity(&self) -> SpatiumFunctions {
        let number_spatiums = self.ship_dimensions.number_spatiums();
        let length_spatium = self.ship_dimensions.length_spatium();
        let length_between_perpendiculars = self.ship_dimensions.length_between_perpendiculars();
        let mut spatium_functions = SpatiumFunctions::filled_zeros(number_spatiums, length_spatium, length_between_perpendiculars);
        for shipload in self.shiploads.shiploads() {
            let shipload_intensity = shipload.shipload_intensity(&self.ship_dimensions);
            for spatium_function in shipload_intensity.iter() {
                spatium_functions.add_spatium_function(&spatium_function)
            }
        }
        spatium_functions
    }
}
