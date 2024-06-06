use crate::strength::load::shiploads::Shiploads;

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
