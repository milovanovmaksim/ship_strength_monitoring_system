use log::{warn, debug};
use serde::Deserialize;

use crate::{strength::ship::{spatium_function::SpatiumFunction, ship_dimensions::ShipDimensions, load::shipload::Shipload}, core::json_file::JsonFile};

#[derive(Deserialize, Debug)]
pub struct DeadweightIntensity {
    shiploads: Option<Vec<Shipload>>,
    ship_dimensions: ShipDimensions,
}


impl DeadweightIntensity {
    pub fn new(shiploads: Option<Vec<Shipload>>, ship_dimensions: ShipDimensions,) -> Self {
        DeadweightIntensity { shiploads, ship_dimensions }
    }

    ///
    /// Create the object from json file.
    pub fn from_json_file(file_path: String) -> Result<Self, String> {
        let json = JsonFile::new(file_path);
        match json.content() {
            Ok(content) => {
                match serde_json::from_reader(content) {
                    Ok(deadweight_intensity) => {
                        debug!("DeadweightIntensity::from_json_file | DeadweightIntensity has been created sucessfuly. {:?}", deadweight_intensity);
                        Ok(deadweight_intensity)
                    },
                    Err(err) => {
                        warn!("DeadweightIntensity::from_json_file | error: {:?}.",err);
                        return Err(err.to_string());
                    }
                }
            },
            Err(err) => {
                warn!("DeadweightIntensity::from_json_file | error: {:?}.",err);
                return Err(err);
            }
        }
    }

    pub fn deadweight_intensity(&self) -> Option<Vec<SpatiumFunction>> {
        match &self.shiploads {
            Some(loads) => {
                let deadweight_intensity = self.spatiums_filled_zero();
                for load in loads {
                    let load_intensity = load.load_intensity(&self.ship_dimensions);
                }
                Some(deadweight_intensity)
            },
            None => { None }
        }
    }

    fn spatiums_filled_zero(&self) -> Vec<SpatiumFunction> {
        let length_spatiums = self.ship_dimensions.length_spatium();
        let mut spatiums = vec![];
        let mut current_coordinate = self.ship_dimensions.coordinate_aft();
        for id in 0..self.ship_dimensions.number_spatiums() {
            let end_coordinate = current_coordinate + length_spatiums;
            let spatium = SpatiumFunction::new(id, current_coordinate, end_coordinate, 0.0, 0.0);
            spatiums.push(spatium);
            current_coordinate += length_spatiums;
        }
        spatiums
    }

}