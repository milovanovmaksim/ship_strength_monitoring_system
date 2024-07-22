use std::ops::Div;

use log::{info, warn};
use serde::Deserialize;

use super::json_file::JsonFile;

///
/// Плотность воды.
#[derive(Debug, Clone, Copy, Deserialize)]
pub(crate) struct WaterDensity {
    water_density: f64,
}

impl WaterDensity {
    pub fn new(water_density: f64) -> Self {
        WaterDensity { water_density }
    }

    ///
    /// Возвращает плотность воды.
    pub fn water_density(&self) -> f64 {
        self.water_density
    }
    ///
    /// Create the object from json file.
    pub fn from_json_file(file_path: String) -> Result<Self, String> {
        let json = JsonFile::new(file_path);
        match json.content() {
            Ok(content) => {
                match serde_json::from_reader(content) {
                    Ok(water_density) => {
                        info!("Shiploads::from_json_file | Shiploads has been created sucessfuly. {:?}", water_density);
                        Ok(water_density)
                    }
                    Err(err) => {
                        warn!("Shiploads::from_json_file | error: {:?}.", err);
                        return Err(err.to_string());
                    }
                }
            }
            Err(err) => {
                warn!("Shiploads::from_json_file | error: {:?}.", err);
                return Err(err);
            }
        }
    }
}

impl Div<WaterDensity> for f64 {
    type Output = f64;

    fn div(self, rhs: WaterDensity) -> Self::Output {
        if rhs.water_density() < 0.0 {
            panic!("Деление на ноль.")
        }
        self / rhs.water_density()
    }
}
