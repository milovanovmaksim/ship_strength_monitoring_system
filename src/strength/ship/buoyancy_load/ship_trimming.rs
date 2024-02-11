use crate::strength::ship::deadweight::{deadweight::Deadweight, deadweight_intensity::DeadweightIntensity};



///
/// Удифферентовка судна.
pub(crate) struct ShipTrimming {
    deadweight_intensity: DeadweightIntensity,
    deadweight: Deadweight,
}


impl ShipTrimming {

    ///
    /// Создает новый объект.
    pub fn new(deadweight_intensity: DeadweightIntensity, deadweight: Deadweight) -> Self {
        ShipTrimming { deadweight_intensity, deadweight }
    }

    ///
    /// Удифферентовка судна. Возвращает осадку кормы и носа судна.
    pub fn trimming(self) -> (f64, f64) {
        todo!("Алгоритм удифферентовка судна.");

    }
}