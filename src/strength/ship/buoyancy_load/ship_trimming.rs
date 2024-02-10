use crate::strength::ship::deadweight::{deadweight::Deadweight, deadweight_intensity::DeadweightIntensity};
use super::frames::Frames;


///
/// Удифферентовка судна.
pub(crate) struct ShipTrimming {
    frames: Frames,
    deadweight_intensity: DeadweightIntensity,
    deadweight: Deadweight,
}


impl ShipTrimming {

    ///
    /// Создает новый объект.
    pub fn new(frames: Frames, deadweight_intensity: DeadweightIntensity, deadweight: Deadweight) -> Self {
        ShipTrimming { frames, deadweight_intensity, deadweight }
    }

    ///
    /// Удифферентовка судна. Возвращает осадку кормы и носа судна.
    pub fn trimming(self) -> (f64, f64) {
        todo!("Алгоритм удифферентовка судна.");

    }
}