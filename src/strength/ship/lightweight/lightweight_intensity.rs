
use crate::strength::ship::{ship_dimensions::ShipDimensions, spatium_function::SpatiumFunction, spatium_functions::SpatiumFunctions};


///
/// Lightweight - weight of the empty as-built ship without cargo, fuel, lubricating oil, ballast water,
/// fresh water and feed water in tanks, consumable stores, passengers and crew and their belongings. Measured in tons.
#[derive(Debug)]
pub struct LightweightIntensity {

}

impl LightweightIntensity {
    pub fn new() -> Self {
        LightweightIntensity {  }
    }

    ///
    /// Computes the lightweight intensity for spatiums.
    pub fn spatium_functions(&mut self, lightweight: f64, ship_dimensions: &ShipDimensions) -> SpatiumFunctions {
        let mut lightweight_intensity: Vec<SpatiumFunction> = vec![];
        let half_length_spatium = ship_dimensions.length_spatium() / 2.0;
        let mut current_coord = ship_dimensions.coordinate_aft() + half_length_spatium;
        let (a, b, c) = ship_dimensions.lightweight_intensity_parameters();
        let intensity_load = |ratio: f64| {
            ((lightweight / ship_dimensions.number_spatiums() as f64) * ratio) / ship_dimensions.length_spatium()
        };
        let mut ratio: f64;
        for id in 0..ship_dimensions.number_spatiums() {
            let end_coord = current_coord + half_length_spatium;
            let start_coord = current_coord - half_length_spatium;
            if current_coord > ship_dimensions.coordinate_aft() && current_coord < (ship_dimensions.coordinate_aft() + ship_dimensions.length_between_perpendiculars() / 3.0) {
                ratio = a + ((b - a) * ((ship_dimensions.length_between_perpendiculars() / 2.0) - current_coord.abs()))/(ship_dimensions.length_between_perpendiculars() / 3.0);
            } else if current_coord >= ship_dimensions.coordinate_aft() + ship_dimensions.length_between_perpendiculars() / 3.0 && current_coord < (ship_dimensions.coordinate_bow() - ship_dimensions.length_between_perpendiculars() / 3.0) {
                ratio = b;
            } else {
                ratio = c + ((b - c) * (ship_dimensions.length_between_perpendiculars() / 2.0 - current_coord))/(ship_dimensions.length_between_perpendiculars() / 3.0);
            }
            let f_x = intensity_load(ratio);
            let spatium_function = SpatiumFunction::new(id, start_coord, end_coord, f_x, f_x);
            lightweight_intensity.push(spatium_function);

            current_coord += ship_dimensions.length_spatium();
        }
        SpatiumFunctions::new(lightweight_intensity)
    }
}


