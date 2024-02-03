
use crate::strength::ship::ship_dimensions::ShipDimensions;


///
/// Lightweight - weight of the empty as-built ship without cargo, fuel, lubricating oil, ballast water,
/// fresh water and feed water in tanks, consumable stores, passengers and crew and their belongings. Measured in tons.
#[derive(Debug)]
pub struct LightweightIntensity {
    lightweight_intensity: Option<Vec<(f64, f64)>>

}

impl LightweightIntensity {
    pub fn new(lightweight_intensity: Option<Vec<(f64, f64)>>) -> Self {
        LightweightIntensity { lightweight_intensity }
    }

    ///
    /// Computes the lightweight intensity for spatiums.
    pub fn lightweight_intensity(&mut self, lightweight: f64, ship_dimensions: &ShipDimensions) -> &Vec<(f64, f64)> {
        match self.lightweight_intensity {
            Some(ref intensity) => { return intensity }
            None => {
                let mut intensity: Vec<(f64, f64)> = vec![];
                let half_length_spatium = ship_dimensions.length_spatium() / 2.0;
                let mut current_coord = ship_dimensions.coordinate_aft() + half_length_spatium;
                let (a, b, c) = ship_dimensions.lightweight_intensity_parameters();
                let intensity_load = |ratio: f64| {
                    ((lightweight / ship_dimensions.number_spatiums() as f64) * ratio) / ship_dimensions.length_spatium()
                };
                let mut ratio: f64;
                for _id in 0..ship_dimensions.number_spatiums() {
                    let end_coord = current_coord + half_length_spatium;
                    let start_coord = current_coord - half_length_spatium;
                    if current_coord > ship_dimensions.coordinate_aft() && current_coord < (ship_dimensions.coordinate_aft() + ship_dimensions.length_between_perpendiculars() / 3.0) {
                        ratio = a + ((b - a) * ((ship_dimensions.length_between_perpendiculars() / 2.0) - current_coord.abs()))/(ship_dimensions.length_between_perpendiculars() / 3.0);
                    } else if current_coord >= ship_dimensions.coordinate_aft() + ship_dimensions.length_between_perpendiculars() / 3.0 && current_coord < (ship_dimensions.coordinate_bow() - ship_dimensions.length_between_perpendiculars() / 3.0) {
                        ratio = b;
                    } else {
                        ratio = c + ((b - c) * (ship_dimensions.length_between_perpendiculars() / 2.0 - current_coord))/(ship_dimensions.length_between_perpendiculars() / 3.0);
                    }
                    intensity.push((start_coord, intensity_load(ratio)));
                    intensity.push((end_coord, intensity_load(ratio)));
                    current_coord += ship_dimensions.length_spatium();
                }
                self.lightweight_intensity = Some(intensity);
                self.lightweight_intensity.as_ref().unwrap()
            }
        }
    }
}

