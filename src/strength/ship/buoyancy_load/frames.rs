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
                error!("Frames::new | error: {}", err);
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

    ///
    /// Валидация входных данных.
    fn frames_validate(self) -> Result<Frames, String> {
        if self.frames.len() == 0 {
            return Err("Шпангоуты не заданы.".to_string());
        }
        Ok(self)
    }

    ///
    /// Возвращает первый шпангоут.
    fn first(&self) -> &Frame {
        self.frames.first().unwrap()
    }

    ///
    /// Возвращает последний шпангоут.
    fn last(&self) -> &Frame {
        self.frames.last().unwrap()
    }

    ///
    /// Бинарный поиск шпангоута по абсциссе.
    /// Возвращает найденный шпангоут.
    /// Если шпангоут с заданной абсциссой отсутствует, возвращает соседние шпангоуты между которыми лежит
    /// искомый шпангоут ```(Some(left_frame), Some(right_frame))```. Если абсцисса вышла за пределы корабля,
    /// возвращает (None, None).
    fn frame_by_abscissa(&self, abscissa: f64) -> (Option<&Frame>, Option<&Frame>) {
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
                return (Some(frame), None);
            }
        }
        let left_frame = self.frames.get(left_point).unwrap();
        let right_frame = self.frames.get(right_point).unwrap();
        if abscissa == left_frame.abscissa() {
            return (Some(left_frame), None);
        }
        if abscissa == right_frame.abscissa() {
            return (Some(right_frame), None);
        }
        (Some(left_frame), Some(right_frame))
    }


    fn get(&self, index: usize) -> Option<&Frame> {
        self.frames.get(index)
    }


    ///
    /// Валидация абсциссы.
    /// Если абсцисса вышла за пределы координаты кормы или носа судна, возвращается ошибка.
    /// Parameters:
    ///     abscissa - координата шпангоута относительно центра корабля [м],
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
    ///     abscissa - координата шпангоута относительно центра корабля [м],
    ///     draft - осадка корабля [м].
    fn frame_underwater_area(&self, abscissa: f64, draft: f64) -> Result<f64, String> {
        match self.validate_abscissa(abscissa) {
            Ok(_) => {
                match self.frame_by_abscissa(abscissa) {
                    (Some(frame), None) => {
                        match frame.area_by_draft(draft) {
                            Ok(value) => { Ok(value) }
                            Err(err) => {
                                error!("Frames::frame_underwater_area | error: {}", err);
                                Err(err)
                            }
                        }
                    }
                    (Some(left_frame), Some(right_frame)) => {
                        let left_value = left_frame.area_by_draft(draft);
                        if let Err(err) = left_value {
                            error!("Frames::frame_underwater_area | error: {}", err);
                            return Err(err);
                        }
                        let right_value = right_frame.area_by_draft(draft);
                        if let Err(err) = right_value {
                            error!("Frames::frame_underwater_area | error: {}", err);
                            return Err(err);
                        }
                        let linear_interpolation = LinearInterpolation::new(left_value.unwrap(), right_value.unwrap(),
                            left_frame.abscissa(), right_frame.abscissa());
                        match linear_interpolation.interpolated_value(abscissa) {
                            Ok(value) => { Ok(value) }
                            Err(err) => {
                                error!("Frames::frame_underwater_area | error: {}", err);
                                Err(err)
                            }
                        }
                    }
                    _ => { unreachable!("Абсцисса лежит в диапазоне между координатой кормы и носа."); }
                }
            }
            Err(err) => {
                error!("Frames::frame_underwater_area | error: {}", err);
                Err(err)
            }
        }
    }

    ///
    /// Возвращает погруженный объем шпангоута для заданной осадки и абсциссы. [м^3]
    /// Parameters:
    ///     x - координата шпангоута относительно центра корабля (абсцисса) [м],
    ///     draft - осадка корабля [м].
    pub fn frame_underwater_volume(&self, abscissa: f64, draft: f64, length_spatium: f64) -> Result<f64, String> {
        match self.frame_underwater_area(abscissa, draft) {
            Ok(area) => { Ok((area * length_spatium).my_round(2)) }
            Err(err) => {
                error!("Frames::frame_underwater_volume | error: {}", err);
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