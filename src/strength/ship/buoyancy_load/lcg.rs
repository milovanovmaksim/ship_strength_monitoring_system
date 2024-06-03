use crate::strength::ship::load::shiploads::Shiploads;

pub struct LCG<'a> {
    shiploads: &'a Shiploads

}


impl<'a> LCG<'a> {
    pub fn new(shiploads: &'a Shiploads) -> Self {
        LCG { shiploads }
    }


    pub fn lcg(&self) -> f64 {
        let mut moment = 0.0;
        for shiplod in self.shiploads.as_ref() {
            moment += shiplod.moment();
        }
        moment / self.shiploads.sum()
    }
}