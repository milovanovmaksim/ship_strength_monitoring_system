use log::{debug, warn};
use serde::Deserialize;

use crate::{strength::ship::{ship_dimensions::ShipDimensions, load::shipload::Shipload}, core::json_file::JsonFile};



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


    pub fn shared_loads(&self) {
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
                todo!()

            }

        }

    }

}



