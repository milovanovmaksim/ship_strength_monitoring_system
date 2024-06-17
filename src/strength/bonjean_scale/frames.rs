use crate::core::json_file::JsonFile;
use log::{debug, error, warn};

use super::frame::Frame;

///
/// Шпангоуты судна.
pub struct Frames {
    frames: Vec<Frame>,
}

impl Frames {
    ///
    /// Основной конструктор.
    pub fn new(frames: Vec<Frame>) -> Result<Self, String> {
        match (Frames { frames }).frames_validate() {
            Ok(frames) => Ok(frames),
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
            Ok(content) => match serde_json::from_reader(content) {
                Ok(frames) => {
                    debug!("Frames::from_json_file | Frames has been created sucessfuly.");
                    Frames::new(frames)
                }
                Err(err) => {
                    warn!("Frames::from_json_file | error: {:?}.", err);
                    return Err(err.to_string());
                }
            },
            Err(err) => {
                warn!("Frames::from_json_file | error: {:?}.", err);
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
    pub fn first(&self) -> &Frame {
        self.frames.first().unwrap()
    }

    ///
    /// Возвращает последний шпангоут.
    pub fn last(&self) -> &Frame {
        self.frames.last().unwrap()
    }

    ///
    /// Бинарный поиск шпангоута по абсциссе.
    /// Возвращает найденный шпангоут.
    /// Если шпангоут с заданной абсциссой отсутствует, возвращает соседние шпангоуты между которыми лежит
    /// искомый шпангоут ```(Some(left_frame), Some(right_frame))```. Если абсцисса вышла за пределы корабля,
    /// возвращает (None, None).
    pub fn frame_by_abscissa(&self, abscissa: f64) -> (Option<&Frame>, Option<&Frame>) {
        if abscissa > self.last().abscissa() && abscissa < self.first().abscissa() {
            return (None, None);
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
        let left_frame = self.get(left_point).unwrap();
        let right_frame = self.get(right_point).unwrap();
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
}

impl AsRef<Vec<Frame>> for Frames {
    fn as_ref(&self) -> &Vec<Frame> {
        &self.frames
    }
}
