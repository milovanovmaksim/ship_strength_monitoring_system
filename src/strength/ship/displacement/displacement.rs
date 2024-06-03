use crate::strength::ship::buoyancy_load::bonjean_scale::BonjeanScale;


///
/// Объемное водоизмещение судна.
pub struct Displacement<'a> {
    bonjean_scale: &'a BonjeanScale<'a>
}


impl<'a> Displacement<'a> {

    ///
    /// Конструктор.
    pub fn new(bonjean_scale: &'a BonjeanScale) -> Self {
        Displacement { bonjean_scale }
    }

    ///
    /// Возвращает объемное водоизмещение судна [м^3]
    pub fn displacement(&self) -> Result<f64, String> {
        todo!();
    }
}