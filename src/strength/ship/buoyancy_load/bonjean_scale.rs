use log::{debug, warn};
use serde::Deserialize;

use crate::{core::{json_file::JsonFile, linear_interpolation::LinearInterpolation}, strength::ship::ship_dimensions::ShipDimensions};
use super::frame::Frame;


///
/// Масштаб Бонжана.
/// Parameters:
///     frames: Vec<Frame> - список шпангоутов судна.
#[derive(Deserialize, Debug)]
pub(crate) struct BonjeanScale {
    frames: Vec<Frame>,
    shipdimensions: ShipDimensions
}

impl BonjeanScale {
    pub fn new(frames: Vec<Frame>, shipdimensions: ShipDimensions) -> Self {
        BonjeanScale { frames, shipdimensions }
    }

    ///
    /// Create the object from json file.
    pub fn from_json_file(file_path: String) -> Result<Self, String> {
        let json = JsonFile::new(file_path);
        match json.content() {
            Ok(content) => {
                match serde_json::from_reader(content) {
                    Ok(frames) => {
                        debug!("BonjeanScale::from_json_file | Frames has been created sucessfuly. {:?}", frames);
                        Ok(frames)
                    },
                    Err(err) => {
                        warn!("BonjeanScale::from_json_file | error: {:?}.",err);
                        return Err(err.to_string());
                    }
                }
            },
            Err(err) => {
                warn!("BonjeanScale::from_json_file | error: {:?}.",err);
                return Err(err);
            }
        }
    }

    ///
    /// Бинарный поиск индекса шпангоута (Frame) по абсциссе.
    /// Возвращает индекс найденного шпангоута ```(Some(index), None)```.
    /// Если шпангоут с заданной абсциссой отсутствует, возвращает индексы соседних шпангоутов между которыми лежит
    /// искомый шпангоут ```(Some(index), Some(index))```.
    /// Если шпангоут не найден, возвращает ```(None, None)```.
    fn frame_by_abscissa(&self, abscissa: f64) -> Result<(&Frame, Option<&Frame>), String> {
        match self.check_abscissa(abscissa) {
            Ok(_) => {
                let mut left_point = 0;
                let mut right_point = self.frames.len() - 1;
                while left_point != right_point - 1 {
                    let middle = (left_point + right_point) / 2;
                    let frame = self.frames.get(middle).unwrap();
                    if frame.abscissa() > abscissa {
                        right_point = middle;
                    } else if frame.abscissa() < abscissa {
                        left_point = middle
                    } else if frame.abscissa() == abscissa {
                        return Ok((self.frames.get(middle).unwrap(), None));
                    }
                }
                Ok((self.frames.get(left_point).unwrap(), self.frames.get(right_point)))
            }
            Err(error) => {
                debug!("BonjeanScale::frame_by_abscissa | error: {}", error);
                Err(error)
            }
        }
    }

    fn check_abscissa(&self, abscissa: f64) -> Result<(), String> {
        if abscissa < self.shipdimensions.coordinate_aft() {
            return Err(format!("Абсцисса вышла за пределы координаты кормы судна. Координа кормы: {}. Переданно значение: {}",
                self.shipdimensions.coordinate_aft(), abscissa));
        }
        if abscissa > self.shipdimensions.coordinate_bow() {
            return Err(format!("Абсцисса вышла за пределы координаты носа судна. Координа носа: {}. Переданно значение: {}",
                self.shipdimensions.coordinate_bow(), abscissa));
        }
        Ok(())
    }


    ///
    /// Возвращает погруженный объем шпангоута для заданной осадки и абсциссы. [м^3]
    /// Если шпангоут с заданной абсциссой отсутствует, линейно интерполирует
    /// объем шпангоутов, имея в распоряжение объем двух соседних шпангоутов для заданной осадки.
    /// Parameters:
    ///     x - координата шпангоута относительно центра корабля (абсцисса) [м],
    ///     draft - осадка корабля [м].
    pub fn underwater_volume_frame(&self, abscissa: f64, draft: f64) -> f64 {
        todo!();


    }

    ///
    /// Возвращает погруженную площадь шпангоута для заданной осадки и абсциссы. [м^2]
    /// Если шпангоут с заданной абсциссой отсутствует, линейно инерполирует
    /// площадь шпангоутов, имея в распоряжение площадь двух соседних шпангоутов для заданной осадки.
    /// Parameters:
    ///     x - координата шпангоута относительно центра корабля (абсцисса) [м],
    ///     draft - осадка корабля [м].
    pub fn underwater_area_frame(&self, abscissa: f64, draft: f64) -> f64 {
        todo!();


    }


    ///
    /// Возвращает погруженный объем судна в воду (объемное водоизмещение) для заданных осадок носа и кормы.
    /// Parameters:
    ///     aft_draft - осадка кормы [м],
    ///     nose_draft - осадка носа [м].
    pub fn ship_underwater_volume(&self, aft_draft: f64, nose_draft: f64) -> f64 {
        todo!();


    }
}

impl AsRef<Vec<Frame>> for BonjeanScale {

    fn as_ref(&self) -> &Vec<Frame> {
        &self.frames
    }
}