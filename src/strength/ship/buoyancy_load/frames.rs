use log::{debug, error, warn};
use crate::core::json_file::JsonFile;

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
                        debug!("Frames::from_json_file | Frames has been created sucessfuly. {:?}", frames);
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

    pub fn get(&self, index: usize) -> Option<&Frame> {
        self.frames.get(index)
    }
}


impl AsRef<Vec<Frame>> for Frames {

    fn as_ref(&self) -> &Vec<Frame> {
        &self.frames
    }
}