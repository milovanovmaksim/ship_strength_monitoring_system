use log::{debug, warn};
use serde::Deserialize;

use crate::core::json_file::JsonFile;
use super::frame::Frame;

///
/// Масштаб Бонжана.
/// Parameters:
///     frame: Vec<Frame> - список шпангоутов корабля.
#[derive(Deserialize, Debug)]
pub(crate) struct BonjeanScale {
    frames: Vec<Frame>
}

impl BonjeanScale {
    pub fn new(frames: Vec<Frame>) -> Self {
        BonjeanScale { frames }
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
    /// Бинарный поиск шпангоута по абсциссе.
    /// Возвращает id шпангоута. Если такого нет, то возвращаем ближайший.
    fn frame_by_abscissa(&self, abscissa: f64) -> usize {

    }

    ///
    /// Линейно интерполирует данные масштаба Бонжана по двум известным шпангоутам для заданной координаты (абсциссе).
    /// Возвращает шпангоут Frame, полученный в результате линейной интерполяции.
    /// Parameters:
    ///     abscissa - абсцисса в которой необходимо получить данные масштаба Бонжана.
    fn interpolated_frame(&self, abscissa: f64) -> Frame {

    }

    ///
    /// Возвращает погруженный объем шпангоута для заданной осадки.
    /// Если шпангоут с координатой x отсутствует, линейно инерполируем
    /// объем шпангоута, имея в распоряжение объемы двух соседних.
    /// Parameters:
    ///     x - координата шпангоута относительно центра корабля [м],
    ///     draft - осадка корабля [м].
    pub fn frame_underwater_volume(&self, abscissa: f64, draft: f64) -> f64 {


    }


    ///
    /// Возвращает погруженный объем корабля в воду (объемное водоизмещение) для заданных осадок носа и кормы.
    /// Parameters:
    ///     aft_draft - осадка кормы,
    ///     nose_draft - осадка носа
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