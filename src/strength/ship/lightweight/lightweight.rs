use log::{warn, debug};
use serde::Deserialize;

use crate::{core::json_file::JsonFile, strength::{output::{output::Output, type_output::TypeOutput}, ship::{spatium::Spatium, ship_dimensions::ShipDimensions}}};



///
/// Lightweight - weight of the empty as-built ship without cargo, fuel, lubricating oil, ballast water,
/// fresh water and feed water in tanks, consumable stores, passengers and crew and their belongings. Measured in tons.
#[derive(Deserialize, Debug)]
pub struct LightweightIntensity {
    lightweight: f64,
    ship_dimensions: ShipDimensions,
}

impl LightweightIntensity {
    pub fn new(lightweight: f64, ship_metrics: ShipDimensions) -> Self {
        LightweightIntensity { lightweight, ship_dimensions: ship_metrics}
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
    /// Computes the lightweight intensity for spatiums
    pub fn lightweight_intensity(&self) -> Output {
        let mut spatiums = vec![];
        let half_length_spatium = self.ship_dimensions.length_spatium() / 2.0;
        let mut current_coord = self.ship_dimensions.coordinate_aft() + half_length_spatium;
        for id in 0..self.ship_dimensions.number_spatiums() {
            let spatium = self.spatium(current_coord, half_length_spatium, id);
            spatiums.push(spatium);
            current_coord += self.ship_dimensions.length_spatium();
        }
        debug!("Lightweight.lightweight_intensity() | Lightweight intensity hase been computed successfully.");
        Output::new(spatiums, TypeOutput::LightweightIntensity)

    }
    ///
    /// Computes the lightweight intensity for the spatium.
    fn spatium(&self, current_coord: f64, half_length_spatium: f64, id: i64) -> Spatium {
        let (a, b, c) = self.ship_dimensions.lightweight_intensity_parameters();
        let end_coord = current_coord + half_length_spatium;
        let start_coord = current_coord - half_length_spatium;
        let intensity_load = |parametr: f64| {
            ((self.lightweight / self.ship_dimensions.number_spatiums() as f64) * parametr) / self.ship_dimensions.length_spatium()
        };
        if current_coord > self.ship_dimensions.coordinate_aft() && current_coord < (self.ship_dimensions.coordinate_aft() + self.ship_dimensions.length_between_perpendiculars() / 3.0) {
            let parametr = a + ((b - a) * ((self.ship_dimensions.length_between_perpendiculars() / 2.0) - current_coord.abs()))/(self.ship_dimensions.length_between_perpendiculars() / 3.0);
            let intensity_load = intensity_load(parametr);
            Spatium::new(id, start_coord, end_coord, intensity_load, intensity_load)
        } else if current_coord >= self.ship_dimensions.coordinate_aft() + self.ship_dimensions.length_between_perpendiculars() / 3.0 && current_coord < (self.ship_dimensions.coordinate_bow() - self.ship_dimensions.length_between_perpendiculars() / 3.0) {
            let intensity_load = intensity_load(b);
            Spatium::new(id, start_coord, end_coord, intensity_load, intensity_load)
        } else {
            let parametr = c + ((b - c) * (self.ship_dimensions.length_between_perpendiculars() / 2.0 - current_coord))/(self.ship_dimensions.length_between_perpendiculars() / 3.0);
            let intensity_load = intensity_load(parametr);
            Spatium::new(id, start_coord, end_coord, intensity_load, intensity_load)
        }
    }
}

