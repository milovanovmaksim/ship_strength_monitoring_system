use log::{debug, warn};
use serde::Deserialize;

use crate::core::json_file::JsonFile;
use crate::strength::ship::{spatium_functions::SpatiumFunctions, ship_dimensions::ShipDimensions};
use crate::strength::ship::load::shipload::Shipload;


///
/// Сontains all the loads acting on the ship
#[derive(Deserialize, Debug)]
pub struct Shiploads {
    shiploads: Vec<Shipload>,
    ship_dimensions: ShipDimensions,
}


impl Shiploads {

    ///
    /// Create new object.
    fn new(shiploads: Vec<Shipload>, ship_dimensions: ShipDimensions) -> Self {
        Shiploads { shiploads, ship_dimensions }
    }

    ///
    /// Create the object from json file.
    pub fn from_json_file(file_path: String) -> Result<Self, String> {
        let json = JsonFile::new(file_path);
        match json.content() {
            Ok(content) => {
                match serde_json::from_reader(content) {
                    Ok(shiploads) => {
                        debug!("Shiploads::from_json_file | Shiploads has been created sucessfuly. {:?}", shiploads);
                        Ok(shiploads)
                    },
                    Err(err) => {
                        warn!("Shiploads::from_json_file | error: {:?}.",err);
                        return Err(err.to_string());
                    }
                }
            },
            Err(err) => {
                warn!("Shiploads::from_json_file | error: {:?}.",err);
                return Err(err);
            }
        }
    }

    ///
    /// Compute shiploads intensity.
    pub fn intensity(&self) -> SpatiumFunctions {
        let number_spatiums = self.ship_dimensions.number_spatiums();
        let length_spatium = self.ship_dimensions.length_spatium();
        let mut shaptium_functions = SpatiumFunctions::filled_zeros(number_spatiums, length_spatium);

        for shipload in self.shiploads.iter() {
            let load_intensity = shipload.load_intensity(&self.ship_dimensions);
            let _ = load_intensity.iter().map(|spatium_function| { shaptium_functions.add_spatium_function(spatium_function) });
        }
        shaptium_functions
    }

    ///
    /// Return the shiploads sum.
    pub fn sum(&self) -> f64 {
        let mut sum = 0.0;
        for shipload in self.shiploads.iter() {
            sum += shipload.value();
        }
        sum
    }
}