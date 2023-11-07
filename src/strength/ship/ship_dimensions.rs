use log::{warn, debug};
use serde::Deserialize;

use crate::core::json_file::JsonFile;


///
/// ShipDimensions struct contains ship dimensions.
/// - length_between_perpendiculars - https://en.wikipedia.org/wiki/Length_between_perpendiculars,
/// - completeness_coefficient - коэффициент полноты корабля,
/// - number_spatiums - количество теоретических шпаций,
#[derive(Deserialize, Debug, Clone, Copy)]
pub struct ShipDimensions {
    length_between_perpendiculars: f64,
    number_spatiums: u64,
    completeness_coefficient: f64,
}

impl ShipDimensions {
    pub fn new(length_between_perpendiculars: f64, number_spatiums: u64, completeness_coefficient: f64) -> Self {
        ShipDimensions { length_between_perpendiculars, number_spatiums, completeness_coefficient }
    }

    ///
    /// Return length spatium
    pub fn length_spatium(&self) -> f64 {
        self.length_between_perpendiculars / self.number_spatiums as f64
    }

    ///
    /// Return parameters of the ship hull weight distribution function.
    /// TODO: добавить ссылку на литература, откуда эти формулы были взяты.
    pub fn lightweight_intensity_parameters(&self) -> (f64, f64, f64) {
        if self.completeness_coefficient  <= 0.7 {
            (0.64, 1.20, 0.56)
        } else {
            (0.72, 1.17, 0.6)
        }
    }

    ///
    /// Create the object from json file.
    pub fn from_json_file(file_path: String) -> Result<Self, String> {
        let json = JsonFile::new(file_path);
        match json.content() {
            Ok(content) => {
                match serde_json::from_reader(content) {
                    Ok(ship_metrics) => {
                        debug!("ShipMetrics::from_json_file | ShipMetrics has been created sucessfuly. {:?}", ship_metrics);
                        Ok(ship_metrics)
                    },
                    Err(err) => {
                        warn!("ShipMetrics::from_json_file | error: {:?}.",err);
                        return Err(err.to_string());
                    }
                }
            },
            Err(err) => {
                warn!("ShipMetrics::from_json_file | error: {:?}.",err);
                return Err(err);
            }
        }
    }

    ///
    /// Returns the bow (nose) cordinate of the ship
    pub fn coordinate_bow(&self) -> f64 {
        self.length_between_perpendiculars / 2.0
    }

    ///
    /// Returns the aft cordinate of the ship
    pub fn coordinate_aft(&self) -> f64 {
        - self.length_between_perpendiculars / 2.0
    }

    ///
    /// Return number spatiums.
    pub fn number_spatiums(&self) -> u64 {
        self.number_spatiums
    }

    ///
    /// Return length bettwen perpendiculars
    pub fn length_between_perpendiculars(&self) -> f64 {
        self.length_between_perpendiculars
    }


    ///
    /// Return spatium start coordinate.
    pub fn spatium_start_coordinate(&self, id: i64) -> f64 {
        id as f64 * self.length_spatium() - (self.length_between_perpendiculars() / 2.0)
    }

    ///
    /// Return spatium end coordinate.
    pub fn spatium_end_coordinate(&self, id: i64) -> f64 {
        self.spatium_start_coordinate(id) + self.length_spatium()
    }

    ///
    /// Define the spatium index by coordinate.
    pub fn spatium_index_by_coordinate(&self, x: f64) -> i64 {
        if x < self.coordinate_aft() {
            ((x / self.length_spatium()) + (self.number_spatiums()) as f64 / 2.0) as i64 - 1
        } else {
            ((x / self.length_spatium()) + (self.number_spatiums()) as f64 / 2.0) as i64
        }
    }
}