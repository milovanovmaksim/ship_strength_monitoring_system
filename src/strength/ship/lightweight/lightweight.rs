use log::{error, debug};
use serde::Deserialize;

use crate::{core::json_file::JsonFile, strength::ship::{ship_dimensions::ShipDimensions, spatium_function::SpatiumFunction, spatium_functions::SpatiumFunctions}};



///
/// LightweightIntensity - intensity of weight of the empty as-built ship without cargo, fuel, lubricating oil, ballast water,
/// fresh water and feed water in tanks, consumable stores, passengers and crew and their belongings.
#[derive(Deserialize, Debug)]
pub struct LightweightIntensity {
    lightweight: f64,
    ship_dimensions: ShipDimensions,
}

impl LightweightIntensity {

    ///
    /// Create  a new object.
    pub fn new(lightweight: f64, ship_metrics: ShipDimensions) -> Self {
        LightweightIntensity { lightweight, ship_dimensions: ship_metrics}
    }

    ///
    /// Create a new object from json.
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
                        error!("Lightweight::from_json_file | error: {:?}.",err);
                        return Err(err.to_string());
                    }
                }
            },
            Err(err) => {
                error!("Lightweight::from_json_file | error: {:?}.",err);
                return Err(err);
            }
        }
    }

    ///
    /// Computes the lightweight intensity for spatiums
    pub fn lightweight_intensity(&self) -> SpatiumFunctions {
        let mut spatiums = vec![];
        let half_length_spatium = self.ship_dimensions.length_spatium() / 2.0;
        let mut current_coord = self.ship_dimensions.coordinate_aft() + half_length_spatium;
        for id in 0..self.ship_dimensions.number_spatiums() {
            let (a, b, c) = self.ship_dimensions.lightweight_intensity_parameters();
            let end_coord = current_coord + half_length_spatium;
            let start_coord = current_coord - half_length_spatium;
            let intensity_load = |parametr: f64| {
                ((self.lightweight / self.ship_dimensions.number_spatiums() as f64) * parametr) / self.ship_dimensions.length_spatium()
            };
            if current_coord > self.ship_dimensions.coordinate_aft() && current_coord < (self.ship_dimensions.coordinate_aft() + self.ship_dimensions.length_between_perpendiculars() / 3.0) {
                let parametr = a + ((b - a) * ((self.ship_dimensions.length_between_perpendiculars() / 2.0) - current_coord.abs()))/(self.ship_dimensions.length_between_perpendiculars() / 3.0);
                let intensity_load = intensity_load(parametr);
                let spatium_fuction = SpatiumFunction::new(id, start_coord, end_coord, intensity_load, intensity_load);
                spatiums.push(spatium_fuction);
            } else if current_coord >= self.ship_dimensions.coordinate_aft() + self.ship_dimensions.length_between_perpendiculars() / 3.0 && current_coord < (self.ship_dimensions.coordinate_bow() - self.ship_dimensions.length_between_perpendiculars() / 3.0) {
                let intensity_load = intensity_load(b);
                let spatium_fuction =SpatiumFunction::new(id, start_coord, end_coord, intensity_load, intensity_load);
                spatiums.push(spatium_fuction);
            } else {
                let parametr = c + ((b - c) * (self.ship_dimensions.length_between_perpendiculars() / 2.0 - current_coord))/(self.ship_dimensions.length_between_perpendiculars() / 3.0);
                let intensity_load = intensity_load(parametr);
                let spatium_fuction =SpatiumFunction::new(id, start_coord, end_coord, intensity_load, intensity_load);
                spatiums.push(spatium_fuction);
            }
                current_coord += self.ship_dimensions.length_spatium();
            }
        SpatiumFunctions::new(spatiums)
    }
    
}

