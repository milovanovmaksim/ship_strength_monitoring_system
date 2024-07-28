use crate::strength::load::shiploads::Shiploads;

#[derive(Clone, Copy)]
pub struct Deadweight {
    deadweight: f64,
}

impl Deadweight {
    pub fn new(deadweight: f64) -> Self {
        Deadweight { deadweight }
    }

    pub fn from_shiplods(shiplods: &Shiploads) -> Deadweight {
        Deadweight::new(shiplods.sum())
    }

    pub fn deadweight(&self) -> f64 {
        self.deadweight
    }
}
