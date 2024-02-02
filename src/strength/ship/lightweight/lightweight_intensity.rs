
use crate::strength::ship::ship_dimensions::ShipDimensions;


///
/// Lightweight - weight of the empty as-built ship without cargo, fuel, lubricating oil, ballast water,
/// fresh water and feed water in tanks, consumable stores, passengers and crew and their belongings. Measured in tons.
#[derive(Debug)]
pub struct LightweightIntensity {
    lightweight: f64,
    ship_dimensions: ShipDimensions,
}

impl LightweightIntensity {
    pub fn new(lightweight: f64, ship_metrics: ShipDimensions) -> Self {
        LightweightIntensity { lightweight, ship_dimensions: ship_metrics}
    }

    ///
    /// Computes the lightweight intensity for spatiums.
    pub fn intensity(&self) -> Vec<(f64, f64)> {
        let mut intensity: Vec<(f64, f64)> = vec![];
        let half_length_spatium = self.ship_dimensions.length_spatium() / 2.0;
        let mut current_coord = self.ship_dimensions.coordinate_aft() + half_length_spatium;
        let (a, b, c) = self.ship_dimensions.lightweight_intensity_parameters();
        let intensity_load = |ratio: f64| {
            ((self.lightweight / self.ship_dimensions.number_spatiums() as f64) * ratio) / self.ship_dimensions.length_spatium()
        };
        let mut ratio: f64;
        for _id in 0..self.ship_dimensions.number_spatiums() {
            let end_coord = current_coord + half_length_spatium;
            let start_coord = current_coord - half_length_spatium;
            if current_coord > self.ship_dimensions.coordinate_aft() && current_coord < (self.ship_dimensions.coordinate_aft() + self.ship_dimensions.length_between_perpendiculars() / 3.0) {
                ratio = a + ((b - a) * ((self.ship_dimensions.length_between_perpendiculars() / 2.0) - current_coord.abs()))/(self.ship_dimensions.length_between_perpendiculars() / 3.0);
            } else if current_coord >= self.ship_dimensions.coordinate_aft() + self.ship_dimensions.length_between_perpendiculars() / 3.0 && current_coord < (self.ship_dimensions.coordinate_bow() - self.ship_dimensions.length_between_perpendiculars() / 3.0) {
                ratio = b;
            } else {
                ratio = c + ((b - c) * (self.ship_dimensions.length_between_perpendiculars() / 2.0 - current_coord))/(self.ship_dimensions.length_between_perpendiculars() / 3.0);
            }
            intensity.push((start_coord, intensity_load(ratio)));
            intensity.push((end_coord, intensity_load(ratio)));
            current_coord += self.ship_dimensions.length_spatium();
        }
        intensity
    }
}

