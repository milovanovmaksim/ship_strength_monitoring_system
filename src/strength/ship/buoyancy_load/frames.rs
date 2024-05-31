use log::{debug, error, warn};
use crate::core::{json_file::JsonFile, linear_interpolation::LinearInterpolation, round::Round};

use super::frame::Frame;


pub struct Frames {
    frames: Vec<Frame>
}


impl Frames {
    pub fn new(frames: Vec<Frame>) -> Result<Self, String> {
        match (Frames { frames }).frames_validate() {
            Ok(frames) => { Ok(frames) }
            Err(err) => {
                error!("BonjeanScale::new | error: {}", err);
                Err(err)
            }
        }
    }

    ///
    /// Create the object from json file.
    pub fn from_json_file(file_path: String) -> Result<Frames, String> {
        let json = JsonFile::new(file_path);
        match json.content() {
            Ok(content) => {
                match serde_json::from_reader(content) {
                    Ok(frames) => {
                        debug!("Frames::from_json_file | Frames has been created sucessfuly.");
                        Frames::new(frames)
                    },
                    Err(err) => {
                        warn!("Frames::from_json_file | error: {:?}.",err);
                        return Err(err.to_string());
                    }
                }
            },
            Err(err) => {
                warn!("Frames::from_json_file | error: {:?}.",err);
                return Err(err);
            }
        }
    }

    fn frames_validate(self) -> Result<Frames, String> {
        if self.frames.len() == 0 {
            return Err("Шпангоуты не заданы.".to_string());
        }
        Ok(self)
    }

    fn first(&self) -> &Frame {
        self.frames.first().unwrap()
    }

    fn last(&self) -> &Frame {
        self.frames.last().unwrap()
    }

    ///
    /// Бинарный поиск индекса шпангоута по абсциссе.
    /// Возвращает индекс найденного шпангоута.
    /// Если шпангоут с заданной абсциссой отсутствует, возвращает индексы соседних шпангоутов между которыми лежит
    /// искомый шпангоут ```(Some(left_point), Some(right_point))```. Если абсцисса вышла за пределы корабля,
    /// возвращает (None, None).
    fn frame_id_by_abscissa(&self, abscissa: f64) -> (Option<usize>, Option<usize>) {
        if abscissa > self.last().abscissa() && abscissa < self.first().abscissa() {
            return (None, None)
        }
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
                return (Some(middle), None);
            }
        }
        let left_frame = self.frames.get(left_point).unwrap();
        let right_frame = self.frames.get(right_point).unwrap();
        if abscissa == left_frame.abscissa() {
            return (Some(left_point), None);
        }
        if abscissa == right_frame.abscissa() {
            return (Some(right_point), None);
        }
        (Some(left_point), Some(right_point))
    }


    fn get(&self, index: usize) -> Option<&Frame> {
        self.frames.get(index)
    }


    fn validate_abscissa(&self, abscissa: f64) -> Result<(), String> {
        if abscissa < self.first().abscissa() {
            return Err(format!("Абсцисса вышла за пределы координаты кормы судна. Координа кормы: {}. Передано значение: {}",
                self.first().abscissa(), abscissa));
        }
        if abscissa > self.last().abscissa() {
            return Err(format!("Абсцисса вышла за пределы координаты носа судна. Координа носа: {}. Передано значение: {}",
                self.last().abscissa(), abscissa));
        }
        Ok(())
    }

    ///
    /// Возвращает погруженную площадь шпангоута для заданной осадки и абсциссы. [м^2]
    /// Если шпангоут с заданной абсциссой отсутствует, линейно инерполирует
    /// площадь шпангоутов, имея в распоряжение площадь двух соседних шпангоутов для заданной осадки.
    /// Parameters:
    ///     x - координата шпангоута относительно центра корабля (абсцисса) [м],
    ///     draft - осадка корабля [м].
    fn underwater_area_frame(&self, abscissa: f64, draft: f64) -> Result<f64, String> {
        match self.validate_abscissa(abscissa) {
            Ok(_) => {
                match self.frame_id_by_abscissa(abscissa) {
                    (Some(index), None) => {
                        let frame = self.get(index).unwrap();
                        match frame.area_by_draft(draft) {
                            Ok(value) => { Ok(value) }
                            Err(err) => {
                                error!("Frames::underwater_area_frame | error: {}", err);
                                Err(err)
                            }
                        }
                    }
                    (Some(left_index), Some(right_index)) => {
                        let left_frame = self.frames.get(left_index).unwrap();
                        let right_frame  = self.frames.get(right_index).unwrap();
                        let left_value = left_frame.area_by_draft(draft);
                        if let Err(err) = left_value {
                            error!("Frames::underwater_area_frame | error: {}", err);
                            return Err(err);
                        }
                        let right_value = right_frame.area_by_draft(draft);
                        if let Err(err) = right_value {
                            error!("Frames::underwater_area_frame | error: {}", err);
                            return Err(err);
                        }
                        let linear_interpolation = LinearInterpolation::new(left_value.unwrap(), right_value.unwrap(),
                            left_frame.abscissa(), right_frame.abscissa());
                        match linear_interpolation.interpolated_value(abscissa) {
                            Ok(value) => { Ok(value) }
                            Err(err) => {
                                error!("Frames::underwater_area_frame | error: {}", err);
                                Err(err)
                            }
                        }
                    }
                    _ => { unreachable!("Абсцисса лежит в диапазоне между координатой кормы и носа."); }
                }
            }
            Err(err) => {
                error!("Frames::underwater_area_frame | error: {}", err);
                Err(err)
            }
        }
    }

    ///
    /// Возвращает погруженный объем шпангоута для заданной осадки и абсциссы. [м^3]
    /// Если шпангоут с заданной абсциссой отсутствует, линейно интерполирует
    /// объем шпангоутов, имея в распоряжение объем двух соседних шпангоутов для заданной осадки.
    /// Parameters:
    ///     x - координата шпангоута относительно центра корабля (абсцисса) [м],
    ///     draft - осадка корабля [м].
    pub fn underwater_volume_frame(&self, abscissa: f64, draft: f64, length_spatium: f64) -> Result<f64, String> {
        match self.underwater_area_frame(abscissa, draft) {
            Ok(area) => { Ok((area * length_spatium).my_round(2)) }
            Err(err) => {
                error!("Frames::underwater_volume_frame | error: {}", err);
                Err(err)
            }
        }
    }
}


impl AsRef<Vec<Frame>> for Frames {

    fn as_ref(&self) -> &Vec<Frame> {
        &self.frames
    }
}