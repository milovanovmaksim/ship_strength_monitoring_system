use log::{debug, error};
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
    pub fn new(frames: Vec<Frame>, shipdimensions: ShipDimensions) -> Result<Self, String> {
        match (BonjeanScale { frames, shipdimensions }).frames_validate() {
            Ok(bonjean_scale) => { Ok(bonjean_scale) }
            Err(err) => {
                error!("BonjeanScale::new | error: {}", err);
                Err(err)
            }
        }
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
                        error!("BonjeanScale::from_json_file | error: {:?}.",err);
                        Err(err.to_string())
                    }
                }
            },
            Err(err) => {
                error!("BonjeanScale::from_json_file | error: {:?}.",err);
                Err(err)
            }
        }
    }

    fn frames_validate(self) -> Result<BonjeanScale, String> {
        if self.frames.len() == 0 {
            return Err("Вектор шпангоутов пуст.".to_string());
        }
        Ok(self)
    }

    ///
    /// Бинарный поиск индекса шпангоута по абсциссе.
    /// Возвращает индекс найденного шпангоута ```(index, None)```.
    /// Если шпангоут с заданной абсциссой отсутствует, возвращает индексы соседних шпангоутов между которыми лежит
    /// искомый шпангоут ```(left_point, Some(right_point))```.
    fn frame_id_by_abscissa(&self, abscissa: f64) -> (usize, Option<usize>) {
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
                return (middle, None);
            }
        }
        let left_frame = self.frames.get(left_point).unwrap();
        let right_frame = self.frames.get(right_point).unwrap();
        if abscissa == left_frame.abscissa() {
            return (left_point, None);
        }
        if abscissa == right_frame.abscissa() {
            return (right_point, None);
        }
        (left_point, Some(right_point))
    }

    fn validate_abscissa(&self, abscissa: f64) -> Result<(), String> {
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
    pub fn underwater_volume_frame(&self, abscissa: f64, draft: f64) -> Result<f64, String> {
        match self.validate_abscissa(abscissa) {
            Ok(_) => {
                match self.frame_id_by_abscissa(abscissa) {
                    (index, None) => {
                        let frame = self.frames.get(index).unwrap();
                        match frame.volume_by_draft(draft) {
                            Ok(volume) => { Ok(volume) }
                            Err(err) => {
                                error!("BonjeanScale::underwater_volume_frame | error: {}", err);
                                Err(err)
                            }
                        }
                    }
                    (left_index, Some(right_index)) => {
                        let left_frame = self.frames.get(left_index).unwrap();
                        let right_frame  = self.frames.get(right_index).unwrap();
                        let left_volume = left_frame.volume_by_draft(draft);
                        if let Err(err) = left_volume {
                            error!("BonjeanScale::underwater_volume_frame | error: {}", err);
                            return Err(err);
                        }
                        let right_volume = right_frame.volume_by_draft(draft);
                        if let Err(err) = left_volume {
                            error!("BonjeanScale::underwater_volume_frame | error: {}", err);
                            return Err(err);
                        }
                        let linear_interpolation = LinearInterpolation::new(left_volume.unwrap(), right_volume.unwrap(),
                            left_frame.abscissa(), right_frame.abscissa());
                        match linear_interpolation.interpolated_value(abscissa) {
                            Ok(volume) => { Ok(volume) }
                            Err(err) => {
                                error!("BonjeanScale::underwater_volume_frame | error: {}", err);
                                Err(err)
                            }
                        }
                    }
                }
            }
            Err(err) => {
                error!("BonjeanScale::underwater_volume_frame | error: {}", err);
                Err(err)
            }
        }
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