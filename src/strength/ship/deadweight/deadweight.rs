use log::{debug, warn};
use serde::Deserialize;
use crate::{core::json_file::JsonFile, strength::ship::load::shiploads::Shiploads};



#[derive(Deserialize, Debug)]
pub struct Deadweight {
    shiploads: Shiploads,

}


impl Deadweight {
    pub fn new(shiploads: Shiploads) -> Self {
        Deadweight { shiploads }
    }

    pub fn deadweight(&self) -> f64 {
        self.shiploads.sum()
    }
}
