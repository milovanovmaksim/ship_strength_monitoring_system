use log::{debug, warn};
use serde::Deserialize;

use crate::core::json_file::JsonFile;

///
/// ShipDimensions struct contains ship dimensions.
/// - length_between_perpendiculars - https://en.wikipedia.org/wiki/Length_between_perpendiculars,
/// - completeness_coefficient - коэффициент полноты корабля,
/// - number_spatiums - количество теоретических шпаций,
#[derive(Deserialize, Debug, Clone, Copy)]
pub struct ShipDimensions {
    lbp: f64,
    number_spatiums: u64,
    completeness_coefficient: f64,
}

impl ShipDimensions {
    pub fn new(lbp: f64, number_spatiums: u64, completeness_coefficient: f64) -> Self {
        ShipDimensions {
            lbp,
            number_spatiums,
            completeness_coefficient,
        }
    }

    ///
    /// Create the object from json file.
    pub fn from_json_file(file_path: String) -> Result<Self, String> {
        let json = JsonFile::new(file_path);
        match json.content() {
            Ok(content) => match serde_json::from_reader(content) {
                Ok(ship_dimensions) => {
                    debug!("ShipDimensions::from_json_file | ShipDimensions has been created sucessfuly. {:?}", ship_dimensions);
                    Ok(ship_dimensions)
                }
                Err(err) => {
                    warn!("ShipDimensions::from_json_file | error: {:?}.", err);
                    return Err(err.to_string());
                }
            },
            Err(err) => {
                warn!("ShipDimensions::from_json_file | error: {:?}.", err);
                return Err(err);
            }
        }
    }

    fn validate_number_spatiums(self) -> Result<ShipDimensions, String> {
        if self.number_spatiums > 200 {
            return Err("Количество шпаций превысило максимально допустимое значение. Максимальное количество шпаций: 200".to_string());
        }
        Ok(self)
    }

    ///
    /// Return length spatium
    pub fn length_spatium(&self) -> f64 {
        self.lbp / self.number_spatiums as f64
    }

    ///
    pub fn completeness_coefficient(&self) -> f64 {
        self.completeness_coefficient
    }

    ///
    /// Returns the bow (nose) cordinate of the ship
    pub fn coordinate_nose(&self) -> f64 {
        self.lbp / 2.0
    }

    ///
    /// Returns the aft cordinate of the ship
    pub fn coordinate_aft(&self) -> f64 {
        -self.lbp / 2.0
    }

    ///
    /// Return number spatiums.
    pub fn number_spatiums(&self) -> u64 {
        self.number_spatiums
    }

    ///
    /// Return length bettwen perpendiculars
    pub fn lbp(&self) -> f64 {
        self.lbp
    }

    ///
    /// Return spatium start coordinate.
    pub fn spatium_start_coordinate(&self, id: u64) -> f64 {
        id as f64 * self.length_spatium() - (self.lbp() / 2.0)
    }

    ///
    /// Return spatium end coordinate.
    pub fn spatium_end_coordinate(&self, id: u64) -> f64 {
        self.spatium_start_coordinate(id) + self.length_spatium()
    }

    ///
    /// Define the spatium index by coordinate.
    pub fn spatium_index_by_coordinate(&self, x: f64) -> u64 {
        // Если координата x выходит за пределы корабля(кормы или носа) необходимо выдывать соответствующую ошибку.
        let mut spatium_start_coordinate = -self.lbp / 2.0;
        let mut spatium_end_coordinate = spatium_start_coordinate + self.length_spatium();
        let mut index = 0;
        for id in 0..self.number_spatiums {
            if x >= spatium_start_coordinate && x < spatium_end_coordinate {
                index = id;
                break;
            }
            spatium_start_coordinate = spatium_end_coordinate;
            spatium_end_coordinate += self.length_spatium();
        }
        index
    }
}
