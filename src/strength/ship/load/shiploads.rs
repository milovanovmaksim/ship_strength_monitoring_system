use crate::strength::ship::{spatium_functions::SpatiumFunctions, ship_dimensions::ShipDimensions};

use super::shipload::Shipload;


///
/// Сontains all the loads acting on the ship
struct Shiploads {
    shiploads: Vec<Shipload>,
    dimensions: ShipDimensions,
}


impl Shiploads {

    ///
    /// Create new object.
    fn new(shiploads: Vec<Shipload>, dimensions: ShipDimensions) -> Self {
        Shiploads { shiploads, dimensions }
    }

    ///
    /// Compute shiploads intensity.
    fn intensity(&self) -> SpatiumFunctions {
        let number_spatiums = self.dimensions.number_spatiums();
        let length_spatium = self.dimensions.length_spatium();
        let mut shaptium_functions = SpatiumFunctions::filled_zeros(number_spatiums, length_spatium);

        for shipload in self.shiploads.iter() {
            let load_intensity = shipload.load_intensity(&self.dimensions);
            let _ = load_intensity.iter().map(|spatium_function| { shaptium_functions.add_spatium_function(spatium_function) });
        }
        shaptium_functions
    }

    ///
    /// Returns the shiploads sum.
    fn sum(&self) -> f64 {
        let mut sum = 0.0;
        for shipload in self.shiploads.iter() {
            sum += shipload.value();
        }
        sum
    }
}