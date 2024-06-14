use crate::strength::load::shiploads::Shiploads;

pub struct Deadweight<'a> {
    shiploads: &'a Shiploads,
}

impl<'a> Deadweight<'a> {
    pub fn new(shiploads: &'a Shiploads) -> Self {
        Deadweight { shiploads }
    }

    pub fn deadweight(&self) -> f64 {
        self.shiploads.sum()
    }
}
