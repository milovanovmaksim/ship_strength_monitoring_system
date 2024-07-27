use std::rc::Rc;

use crate::strength::load::shiploads::Shiploads;

pub struct Deadweight {
    shiploads: Rc<Shiploads>,
}

impl Deadweight {
    pub fn new(shiploads: Rc<Shiploads>) -> Self {
        Deadweight { shiploads }
    }

    pub fn deadweight(&self) -> f64 {
        self.shiploads.sum()
    }
}
