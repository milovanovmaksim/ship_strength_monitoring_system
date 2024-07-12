use crate::strength::{
    deadweight::deadweight_intensity::DeadweightIntensity,
    lightweight::lightweight_intensity::LightweightIntensity,
    ship::spatium_functions::SpatiumFunctions,
};

///
/// Интенсивность водоизмещения судна по его длине.
pub struct DisplacementIntensity<'a> {
    deadweight_intensity: DeadweightIntensity<'a>,
    lightweight_intnesity: LightweightIntensity,
}

impl<'a> DisplacementIntensity<'a> {
    ///
    /// Основной конструктор.
    pub fn new(
        deadweight_intensity: DeadweightIntensity<'a>,
        lightweight_intensity: LightweightIntensity,
    ) -> Self {
        DisplacementIntensity {
            deadweight_intensity,
            lightweight_intnesity: lightweight_intensity,
        }
    }

    ///
    /// Возвращает интенсивность водоизмещения судна по его длине т/м.
    /// Интенсивность водоизмещения определяется как алгебраическая сумма
    /// интенсивностей дедвейта и массы корпуса судна по его длине.
    pub fn spatium_functions(&self) -> SpatiumFunctions {
        let deadweight_intensity = self.deadweight_intensity.deadweight_intensity();
        let lightweight_intnesity = self.lightweight_intnesity.lightweight_intensity();
        let spatium_functions = deadweight_intensity
            .into_iter()
            .zip(lightweight_intnesity.as_ref())
            .map(|spatium_functions| spatium_functions.0.add(spatium_functions.1.clone()))
            .collect();
        SpatiumFunctions::new(spatium_functions)
    }
}
