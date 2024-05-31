
use crate::{core::round::Round, strength::ship::{ship_dimensions::ShipDimensions, spatium_function::SpatiumFunction, spatium_functions::SpatiumFunctions}};

use super::lightweight::Lightweight;


///
/// Lightweight - weight of the empty as-built ship without cargo, fuel, lubricating oil, ballast water,
/// fresh water and feed water in tanks, consumable stores, passengers and crew and their belongings. Measured in tons.
#[derive(Debug)]
pub struct LightweightIntensity<'a> {
    ship_dimensions: &'a ShipDimensions,
    lightweight: Lightweight

}

impl<'a> LightweightIntensity<'a> {
    pub fn new(ship_dimensions: &'a ShipDimensions, lightweight: Lightweight) -> Self {
        LightweightIntensity { ship_dimensions, lightweight }
    }

    ///
    /// Computes the lightweight intensity for spatiums.
    pub fn spatium_functions(&self) -> SpatiumFunctions {
        let mut lightweight_intensity: Vec<SpatiumFunction> = vec![];
        let half_length_spatium = self.ship_dimensions.length_spatium() / 2.0;
        let mut current_coord = self.ship_dimensions.coordinate_aft() + half_length_spatium;
        let (a, b, c) = self.ship_dimensions.lightweight_intensity_parameters();
        let intensity_load = |ratio: f64| {
            ((self.lightweight.lightweight() / self.ship_dimensions.number_spatiums() as f64) * ratio) / self.ship_dimensions.length_spatium()
        };
        let mut ratio: f64;
        for id in 0..self.ship_dimensions.number_spatiums() {
            let end_coord = current_coord + half_length_spatium;
            let start_coord = current_coord - half_length_spatium;
            if current_coord > self.ship_dimensions.coordinate_aft() && current_coord < (self.ship_dimensions.coordinate_aft() + self.ship_dimensions.length_between_perpendiculars() / 3.0) {
                ratio = a + ((b - a) * ((self.ship_dimensions.length_between_perpendiculars() / 2.0) - current_coord.abs()))/(self.ship_dimensions.length_between_perpendiculars() / 3.0);
            } else if current_coord >= self.ship_dimensions.coordinate_aft() + self.ship_dimensions.length_between_perpendiculars() / 3.0 && current_coord < (self.ship_dimensions.coordinate_bow() - self.ship_dimensions.length_between_perpendiculars() / 3.0) {
                ratio = b;
            } else {
                ratio = c + ((b - c) * (self.ship_dimensions.length_between_perpendiculars() / 2.0 - current_coord))/(self.ship_dimensions.length_between_perpendiculars() / 3.0);
            }
            let f_x = intensity_load(ratio).my_round(2);
            let spatium_function = SpatiumFunction::new(id, start_coord.my_round(2), end_coord.my_round(2), f_x, f_x);
            lightweight_intensity.push(spatium_function);

            current_coord += self.ship_dimensions.length_spatium();
        }
        SpatiumFunctions::new(lightweight_intensity)
    }
}


