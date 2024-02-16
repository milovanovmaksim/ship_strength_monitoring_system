use log::{debug, warn};
use serde::Deserialize;

use crate::{core::json_file::JsonFile, strength::ship::ship_dimensions::ShipDimensions};
use super::frame::Frame;


///
/// Масштаб Бонжана.
/// Parameters:
///     frame: Vec<Frame> - список шпангоутов корабля.
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
    /// Бинарный поиск id шпангоутов по абсциссе.
    /// Возвращает id шпангоутов по абсциссе. Если шпангоута с заданной абсциссой нет,
    /// то возвращаем id двух ближайших шпангоутов Ok((Some(id1), Some(id2))),
    /// если такой шпангоут существет, возвращаем его id Ok(Some(id), None).
    fn frame_id_by_abscissa(&self, abscissa: f64) -> Result<(Option<usize>, Option<usize>), String> {
        // TODO: возвращать не id шпангоутов а заимствованные ссылки на шпангоуты &Frame.
        if abscissa >= self.shipdimensions.coordinate_aft() && abscissa <= self.shipdimensions.coordinate_bow() {
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
                    return Ok((Some(middle), None));
                }
            }
            Ok((Some(left_point), Some(right_point)))
        } else {
            Err(format!("Абсцисса вышла за пределы координаты кормы или носа корабля. Абсцисса должна входить в диапозон между {} и {} метров",
                self.shipdimensions.coordinate_aft(), self.shipdimensions.coordinate_bow()))
        }
    }

    ///
    /// Линейно интерполирует данные масштаба Бонжана по двум известным шпангоутам для заданной координаты (абсциссе).
    /// Возвращает шпангоут Frame, полученный в результате линейной интерполяции.
    /// Parameters:
    ///     abscissa - абсцисса в которой необходимо получить данные масштаба Бонжана [м].
    fn interpolated_frame(&self, frame_0: &Frame, frame_1: &Frame) -> Frame {

    }

    ///
    /// Возвращает погруженный объем шпангоута для заданной осадки [м^3].
    /// Если шпангоут с координатой x отсутствует, линейно интерполируем
    /// объем шпангоута, имея в распоряжение объемы двух соседних.
    /// Parameters:
    ///     x - координата шпангоута относительно центра корабля [м],
    ///     draft - осадка корабля [м].
    pub fn underwater_volume_frame(&self, abscissa: f64, draft: f64) -> f64 {


    }

    ///
    /// Возвращает погруженную площаль шпангоута для заданной осадки [м^2].
    /// Если шпангоут с координатой x отсутствует, линейно инерполируем
    /// площадь шпангоута, имея в распоряжение площадь двух соседних.
    /// Parameters:
    ///     x - координата шпангоута относительно центра корабля [м],
    ///     draft - осадка корабля [м].
    pub fn underwater_area_frame(&self, abscissa: f64, draft: f64) -> f64 {


    }


    ///
    /// Возвращает погруженный объем корабля в воду (объемное водоизмещение) для заданных осадок носа и кормы.
    /// Parameters:
    ///     aft_draft - осадка кормы [м],
    ///     nose_draft - осадка носа [м].
    pub fn ship_underwater_volume(&self, aft_draft: f64, nose_draft: f64) -> f64 {


    }
}

impl IntoIterator for BonjeanScale {
    type Item = Frame;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.frames.into_iter()
    }
}

impl AsRef<Vec<Frame>> for BonjeanScale {

    fn as_ref(&self) -> &Vec<Frame> {
        &self.frames
    }
}