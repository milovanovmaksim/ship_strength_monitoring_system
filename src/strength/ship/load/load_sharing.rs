use log::{debug, warn};
use serde::Deserialize;

use crate::{strength::ship::{ship_dimensions::ShipDimensions, load::shipload::Shipload}, core::{json_file::JsonFile, point::Point}};



#[derive(Deserialize, Debug)]
pub struct LoadSharing {
    ship_dimensions: ShipDimensions,
    shiploads: Vec<Shipload>
}


impl LoadSharing {
    pub fn new(ship_dimensions: ShipDimensions, shiploads: Vec<Shipload>) -> Self {
        LoadSharing { ship_dimensions, shiploads }
    }

    ///
    /// Create the object from json file.
    pub fn from_json_file(file_path: String) -> Result<Self, String> {
        let json = JsonFile::new(file_path);
        match json.content() {
            Ok(content) => {
                match serde_json::from_reader(content) {
                    Ok(load_sharing) => {
                        Ok(load_sharing)
                    },
                    Err(err) => {
                        warn!("LoadSharing::from_json_file | error: {:?}.",err);
                        return Err(err.to_string());
                    }
                }
            },
            Err(err) => {
                warn!("LoadSharing::from_json_file | error: {:?}.",err);
                return Err(err);
            }
        }
    }

    ///
    /// Pinch off the shipload.
    /// Params:
        /// load_start_coordinate - shipload start coordinate.
        /// load_end_coordinate - shipload end coordinate.
    /// Return: Shipload.
    fn shared_shipload(&self, load_start_coordinate: f64, load_end_coordinate: f64, shipload: &Shipload) -> Shipload {
        let load_length = (load_start_coordinate.abs() - load_end_coordinate.abs()).abs();
        let center_gravity = shipload.center_gravity();
        let load_value = (load_length / shipload.length()) * shipload.value();
        Shipload::new(load_value, center_gravity, load_length)
    }

    ///
    /// Share the shipload by spatiums.
    pub fn shared_loads(&self) -> Vec<Shipload> {
        let mut shared_shiploads = vec![];
        for shipload in self.shiploads.iter() {
            let x_1 = shipload.load_start_coordinate();
            let x_4 = shipload.load_end_coordinate();
            let spatium_start_index = self.ship_dimensions.spatium_index_by_coordinate(x_1);
            let spatium_end_index = self.ship_dimensions.spatium_index_by_coordinate(x_4);
            let x_2 = self.ship_dimensions.spatium_end_coordinate(spatium_start_index);
            let x_3 = self.ship_dimensions.spatium_start_coordinate(spatium_end_index);
            debug!("x_1 = {}, x_2 = {}, x_3 = {}, x_4 = {}", x_1, x_2, x_3, x_4);
            if (spatium_end_index.abs() - spatium_start_index.abs()).abs() >= 1 {
                let shared_shipload = shipload.clone();
                shared_shiploads.push(shared_shipload);

            } else {
                if (x_1.abs() - x_2.abs()).abs() > 0.0 {
                    let load = self.shared_shipload(x_1, x_2, shipload);
                    shared_shiploads.push(load);
                }
                if (x_4.abs() - x_3.abs()).abs() > 0.0 {
                    let load = self.shared_shipload(x_3, x_4, shipload);
                    shared_shiploads.push(load);
                }
                let mut load_start_coordinate = x_2;
                let mut load_end_coordinate = x_2 + self.ship_dimensions.length_spatium();
                let number_whole_spatiums_under_load = ((x_2.abs() - x_3.abs()).abs() / self.ship_dimensions.length_spatium()) as u64;
                for _ in 0..number_whole_spatiums_under_load {
                    let load = self.shared_shipload(load_start_coordinate, load_end_coordinate, shipload);
                    shared_shiploads.push(load);
                    load_start_coordinate += self.ship_dimensions.length_spatium();
                    load_end_coordinate += self.ship_dimensions.length_spatium();
                }
            }
        }
        shared_shiploads

    }

}



