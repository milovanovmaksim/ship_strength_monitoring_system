use super::lightweight::Lightweight;
use crate::{
    core::{json_file::JsonFile, round::Round},
    strength::ship::{
        ship_dimensions::ShipDimensions, spatium_function::SpatiumFunction,
        spatium_functions::SpatiumFunctions,
    },
};
use log::{error, info};
use serde::Deserialize;

///
/// Интенсивность массы корпуса судна по длине.
#[derive(Debug, Deserialize)]
pub struct LightweightIntensity {
    lightweight_intensity: SpatiumFunctions,
}

impl LightweightIntensity {
    ///
    /// Основной контсруктор.
    pub fn new(lightweight_intensity: SpatiumFunctions) -> Self {
        LightweightIntensity {
            lightweight_intensity,
        }
    }

    ///
    /// Вспомогательный конструктор.
    /// Создает объект из данных о судне [Прочность корабля Курдюмов А.А.].
    pub fn from_ship_input_data(
        ship_dimensions: &ShipDimensions,
        lightweight: &Lightweight,
    ) -> LightweightIntensity {
        let mut lightweight_intensity: Vec<SpatiumFunction> = vec![];
        let half_length_spatium = ship_dimensions.length_spatium() / 2.0;
        let mut current_coord = ship_dimensions.coordinate_aft() + half_length_spatium;
        let (a, b, c) = LightweightIntensity::lightweight_intensity_parameters(
            ship_dimensions.completeness_coefficient(),
        );
        let intensity_load = |ratio: f64| {
            ((lightweight.lightweight() / ship_dimensions.number_spatiums() as f64) * ratio)
                / ship_dimensions.length_spatium()
        };
        let mut ratio: f64;
        for id in 0..ship_dimensions.number_spatiums() {
            let end_coord = current_coord + half_length_spatium;
            let start_coord = current_coord - half_length_spatium;
            if current_coord > ship_dimensions.coordinate_aft()
                && current_coord < (ship_dimensions.coordinate_aft() + ship_dimensions.lbp() / 3.0)
            {
                ratio = a
                    + ((b - a) * ((ship_dimensions.lbp() / 2.0) - current_coord.abs()))
                        / (ship_dimensions.lbp() / 3.0);
            } else if current_coord
                >= ship_dimensions.coordinate_aft() + ship_dimensions.lbp() / 3.0
                && current_coord < (ship_dimensions.coordinate_nose() - ship_dimensions.lbp() / 3.0)
            {
                ratio = b;
            } else {
                ratio = c
                    + ((b - c) * (ship_dimensions.lbp() / 2.0 - current_coord))
                        / (ship_dimensions.lbp() / 3.0);
            }
            let f_x = intensity_load(ratio).my_round(2);
            let spatium_function =
                SpatiumFunction::new(id, start_coord.my_round(2), end_coord.my_round(2), f_x, f_x);
            lightweight_intensity.push(spatium_function);

            current_coord += ship_dimensions.length_spatium();
        }
        LightweightIntensity::new(SpatiumFunctions::new(lightweight_intensity))
    }

    ///
    /// Вспомогательный конструктор.
    pub fn from_json_file(file_path: String) -> Result<LightweightIntensity, String> {
        let json = JsonFile::new(file_path);
        match json.content() {
            Ok(content) => match serde_json::from_reader(content) {
                Ok(lightweight_intensity) => {
                    info!("LightweightIntensity::from_json_file | LightweightIntensity has been created sucessfuly.");
                    return lightweight_intensity;
                }
                Err(err) => {
                    error!("LightweightIntensity::from_json_file | error: {:?}.", err);
                    return Err(err.to_string());
                }
            },
            Err(err) => {
                error!("LightweightIntensity::from_json_file | error: {:?}.", err);
                return Err(err);
            }
        }
    }

    ///
    /// Интенсивность массы корпуса судна по длине [т/м].
    pub fn lightweight_intensity(&self) -> &SpatiumFunctions {
        &self.lightweight_intensity
    }

    ///
    /// Параметры распределения массы корпуса судна по его длине [Прочность корабля Курдюмов А.А.].
    fn lightweight_intensity_parameters(completeness_coefficient: f64) -> (f64, f64, f64) {
        if completeness_coefficient <= 0.7 {
            (0.64, 1.20, 0.56)
        } else {
            (0.72, 1.17, 0.6)
        }
    }
}
