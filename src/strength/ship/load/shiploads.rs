use crate::strength::ship::{spatium_functions::SpatiumFunctions, ship_dimensions::ShipDimensions};

use super::shipload::Shipload;

struct Shiploads {
    shiploads: Vec<Shipload>,
    dimensions: ShipDimensions,
}


impl Shiploads {
    fn new(shiploads: Vec<Shipload>) -> Self {
        Shiploads { shiploads }
    }

    fn intensity(&self) -> SpatiumFunctions {
        let number_spatiums = self.dimensions.number_spatiums();
        let length_spatium = self.dimensions.length_spatium();
        let mut shaptium_functions = SpatiumFunctions::filled_zeros(number_spatiums, length_spatium);

        for shipload in self.shiploads {
            let spatium_function = shipload.load_intensity(&self.ship_demensions);
            shaptium_functions.add_spatium_function(spatium_function);
        }
        shaptium_functions
    }
}