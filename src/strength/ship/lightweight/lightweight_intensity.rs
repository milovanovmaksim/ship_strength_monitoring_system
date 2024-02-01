use log::{warn, debug};
use serde::Deserialize;

use crate::{core::json_file::JsonFile, strength::ship::ship_dimensions::ShipDimensions};


///
/// Lightweight - weight of the empty as-built ship without cargo, fuel, lubricating oil, ballast water,
/// fresh water and feed water in tanks, consumable stores, passengers and crew and their belongings. Measured in tons.
#[derive(Deserialize, Debug)]
pub struct LightweightIntensity {
    lightweight: f64,
    ship_dimensions: ShipDimensions,
    lightweight_intensity: Option<Vec<(f64, f64)>>
}

impl LightweightIntensity {
    pub fn new(lightweight: f64, ship_metrics: ShipDimensions, lightweight_intensity: Option<Vec<(f64, f64)>>) -> Self {
        LightweightIntensity { lightweight, ship_dimensions: ship_metrics, lightweight_intensity}
    }

    pub fn from_json_file(file_path: String) -> Result<Self, String> {
        let json = JsonFile::new(file_path);
        match json.content() {
            Ok(content) => {
                match serde_json::from_reader(content) {
                    Ok(lightweight) => {
                        debug!("Lightweight::from_json_file | Lightweight has been created sucessfuly. {:?}", lightweight);
                        Ok(lightweight)
                    },
                    Err(err) => {
                        warn!("Lightweight::from_json_file | error: {:?}.",err);
                        return Err(err.to_string());
                    }
                }
            },
            Err(err) => {
                warn!("Lightweight::from_json_file | error: {:?}.",err);
                return Err(err);
            }
        }
    }

    ///
    /// Computes the lightweight intensity for spatiums.
    pub fn intensity(&mut self) -> &Vec<(f64, f64)> {
        match self.lightweight_intensity {
            Some(intensity) => { return &intensity; }
            None => {
                let mut intensity: Vec<(f64, f64)> = vec![];
                let half_length_spatium = self.ship_dimensions.length_spatium() / 2.0;
                let mut current_coord = self.ship_dimensions.coordinate_aft() + half_length_spatium;
                let (a, b, c) = self.ship_dimensions.lightweight_intensity_parameters();
                let intensity_load = |ratio: f64| {
                    ((self.lightweight / self.ship_dimensions.number_spatiums() as f64) * ratio) / self.ship_dimensions.length_spatium()
                };
                let mut ratio = 0.0;
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
                    intensity.push((start_coord, intensity_load(ratio)));
                    intensity.push((end_coord, intensity_load(ratio)));
                }
                self.lightweight_intensity = Some(intensity);

            }
        }
        self.lightweight_intensity.as_ref().unwrap()
    }
}

