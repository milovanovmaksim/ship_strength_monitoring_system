use log::{debug, warn};
use serde::Deserialize;
use crate::{core::json_file::JsonFile, strength::ship::{spatium::Spatium, ship_dimensions::ShipDimensions, loads::deck_cargo::deck_cargo::DeckCargo}};



#[derive(Deserialize, Debug)]
pub struct Deadweight {
    deck_cargoes: Option<Vec<DeckCargo>>,
    ship_dimensions: ShipDimensions,
}


impl Deadweight {
    pub fn new(loads: Vec<DeckCargo>, ship_metrics: ShipDimensions) -> Self {
        Deadweight { deck_cargoes: Some(loads) , ship_dimensions: ship_metrics }
    }

    ///
    /// Create the object from json file.
    pub fn from_json_file(file_path: String) -> Result<Self, String> {
        let json = JsonFile::new(file_path);
        match json.content() {
            Ok(content) => {
                match serde_json::from_reader(content) {
                    Ok(deadweight) => {
                        debug!("Deadweight::from_json_file | Deadweight has been created sucessfuly. {:?}", deadweight);
                        Ok(deadweight)
                    },
                    Err(err) => {
                        warn!("Deadweight::from_json_file | error: {:?}.",err);
                        return Err(err.to_string());
                    }
                }
            },
            Err(err) => {
                warn!("Deadweight::from_json_file | error: {:?}.",err);
                return Err(err);
            }
        }
    }

    ///
    /// Сomputes the intensity of deadweight for spatiums. Returns a vector of spatiums (Vec<Spatium>).
    /// Each the spatium in the vector contains intensity of deadweight.
    ///  The intensity of dadweight mesured ton/meter.
    pub fn deadweight_intensity(&self) -> Vec<Spatium> {
        match &self.deck_cargoes {
            Some(loads) => {
                debug!("Deadweight.deadweight_intensity() | Deadweight intensity hase been computed successfully.");
                self.fill_spatiums(loads)
            },
            None => {
                debug!("Deadweight.deadweight_intensity() | Deadweight intensity is empty cause loads had not been set.");
                self.empty_spatiums()
            }
        }
    }
    ///
    /// Fill empty spatiums deadweight intensity.
    fn fill_spatiums(&self, loads: &Vec<DeckCargo>) -> Vec<Spatium> {
        let mut spatiums = self.empty_spatiums();
        let number_spatiums = self.ship_dimensions.number_spatiums();
        let length_spatium = self.ship_dimensions.length_spatium();
        for load in loads {
            let _ = load.intensity(&self.ship_dimensions);
        }
        spatiums

    }

    ///
    /// Returns empty spatiums.
    fn empty_spatiums(&self) -> Vec<Spatium> {
        let length_spatiums = self.ship_dimensions.length_spatium();
        let mut spatiums = vec![];
        let mut current_coordinate = self.ship_dimensions.coordinate_stern();
        for _ in 0..self.ship_dimensions.number_spatiums() {
            let end_coordinate = current_coordinate + length_spatiums;
            let spatium = Spatium::new(current_coordinate, end_coordinate, 0.0, 0.);
            spatiums.push(spatium);
            current_coordinate += length_spatiums;
        }
        spatiums
    }

    pub fn deadweight(&self) -> f64 {
        match &self.deck_cargoes {
            Some(loads) => {
                let mut deadweight = 0.0;
                for load in loads {
                    deadweight += load.weight()
                }
                deadweight
            }
            None => { 0.0 }
        }
    }
}
