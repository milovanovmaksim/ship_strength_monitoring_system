use log::{debug, warn};
use serde::Deserialize;

use crate::core::json_file::JsonFile;
use super::frame::Frame;

///
/// Frames - содержит данные масштаба Бонжана всех шпангоутов судна.
/// Parameters:
///     frame: Vec<Frame> - список шпангоутов корабля.
#[derive(Deserialize, Debug)]
pub(crate) struct Frames {
    frames: Vec<Frame>
}

impl Frames {
    pub fn new(frames: Vec<Frame>) -> Self {
        Frames { frames }
    }

    ///
    /// Create the object from json file.
    pub fn from_json_file(file_path: String) -> Result<Self, String> {
        let json = JsonFile::new(file_path);
        match json.content() {
            Ok(content) => {
                match serde_json::from_reader(content) {
                    Ok(frames) => {
                        debug!("Frames::from_json_file | Frames has been created sucessfuly. {:?}", frames);
                        Ok(frames)
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
}

impl IntoIterator for Frames {
    type Item = Frame;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.frames.into_iter()
    }
}

impl AsRef<Vec<Frame>> for Frames {

    fn as_ref(&self) -> &Vec<Frame> {
        &self.frames
    }
}