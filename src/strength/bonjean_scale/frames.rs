use tracing::instrument;

use crate::core::json_file::JsonFile;

use super::frame::Frame;

///
/// Шпангоуты судна.
#[derive(Debug, PartialEq)]
pub struct Frames {
    frames: Vec<Frame>,
}

impl Frames {
    ///
    /// Основной конструктор.
    #[instrument(target = "Frames::new")]
    pub fn new(frames: Vec<Frame>) -> Result<Self, String> {
        (Frames { frames }).frames_validate()
    }

    ///
    /// Create the object from json file.
    #[instrument(target = "Frames::from_json_file", err)]
    pub fn from_json_file(file_path: String) -> Result<Frames, String> {
        let json = JsonFile::new(file_path);
        let contenst = json.content()?;
        let frames = serde_json::from_reader(contenst).map_err(|e| e.to_string())?;
        Frames::new(frames)
    }

    ///
    /// Валидация входных данных.
    #[instrument(skip(self), err, target = "Frames::frames_validate")]
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
        if abscissa > self.last().abscissa() || abscissa < self.first().abscissa() {
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
