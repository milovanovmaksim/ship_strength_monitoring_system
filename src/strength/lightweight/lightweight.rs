use log::{debug, warn};
use serde::Deserialize;
use tracing::instrument;

use crate::core::json_file::JsonFile;

///
/// Масса судна, когда оно было построено на верфи.
///Не включает массу любого расходного материала, такого как топливо, вода, масло или другие расходные материалы.
#[derive(Deserialize, Debug, Clone, Copy)]
pub struct Lightweight {
    lightweight: f64,
}

impl Lightweight {
    pub fn new(lightweight: f64) -> Self {
        Lightweight { lightweight }
    }

    /// Create the object from json file.
    #[instrument(skip_all, target = "Lightweight::from_json_file")]
    pub fn from_json_file(file_path: String) -> Result<Self, String> {
        let json = JsonFile::new(file_path);
        let content = json.content().map_err(|err| err.to_string())?;
        serde_json::from_reader(content).map_err(|err| err.to_string())
    }

    pub fn lightweight(&self) -> f64 {
        self.lightweight
    }
}
