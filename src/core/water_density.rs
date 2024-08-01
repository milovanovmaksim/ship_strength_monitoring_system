use std::ops::Div;

use serde::Deserialize;
use tracing::instrument;

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

    /// Create the object from json file.
    #[instrument(skip_all, err, target = "WaterDensity::from_json_file")]
    pub fn from_json_file(file_path: String) -> Result<Self, String> {
        let json = JsonFile::new(file_path);
        let content = json.content().map_err(|err| err.to_string())?;
        serde_json::from_reader(content).map_err(|err| err.to_string())
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
