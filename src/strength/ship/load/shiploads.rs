use log::{debug, warn};
use serde::Deserialize;

use crate::core::json_file::JsonFile;
use crate::strength::ship::load::shipload::Shipload;
use crate::strength::ship::ship_dimensions::ShipDimensions;


///
/// Сontains all the loads acting on the ship
#[derive(Deserialize, Debug)]
pub struct Shiploads {
    shiploads: Vec<Shipload>,
}


impl Shiploads {

    ///
    /// Create new object.
    pub fn new(shiploads: Vec<Shipload>) -> Self {
        Shiploads { shiploads }
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

    pub fn shared_shiploads(&self, ship_dimensions: &ShipDimensions) -> Shiploads {
        let mut shiploads = vec![];
        for shipload in self.shiploads.iter() {
            shiploads.extend(shipload.shared_shiploads(ship_dimensions))
        }
        Shiploads::new(shiploads)
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



impl IntoIterator for Shiploads {
    type Item = Shipload;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.shiploads.into_iter()
    }
}

impl AsRef<Vec<Shipload>> for Shiploads {

    fn as_ref(&self) -> &Vec<Shipload> {
        &self.shiploads
    }
}