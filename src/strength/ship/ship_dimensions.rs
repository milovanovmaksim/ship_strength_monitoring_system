use log::{warn, debug};
use serde::Deserialize;

use crate::core::json_file::JsonFile;


///
/// Ship dimensions:
/// - length_between_perpendiculars - https://en.wikipedia.org/wiki/Length_between_perpendiculars,
/// - completeness_coefficient - коэффициент полноты корабля,
/// - number_spatiums - количество теоретических шпаций,
#[derive(Deserialize, Debug, Clone, Copy)]
pub struct ShipDimensions {
    length_between_perpendiculars: f64,
    number_spatiums: i64,
    completeness_coefficient: f64,
}

impl ShipDimensions {
    pub fn new(length_between_perpendiculars: f64, number_spatiums: i64, completeness_coefficient: f64) -> Self {
        ShipDimensions { length_between_perpendiculars, number_spatiums, completeness_coefficient }
    }

    pub fn length_spatium(&self) -> f64 {
        self.length_between_perpendiculars / self.number_spatiums as f64
    }

    pub fn lightweight_intensity_parameters(&self) -> (f64, f64, f64) {
        if self.completeness_coefficient  <= 0.7 {
            (0.65, 1.20, 0.57)
        } else {
            (0.71, 1.17, 0.6)
        }
    }

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

    pub fn coordinate_nose(&self) -> f64 {
        self.length_between_perpendiculars / 2.0
    }

    pub fn coordinate_stern(&self) -> f64 {
        - self.length_between_perpendiculars / 2.0
    }

    pub fn number_spatiums(&self) -> i64 {
        self.number_spatiums
    }

    pub fn length_between_perpendiculars(&self) -> f64 {
        self.length_between_perpendiculars
    }
}