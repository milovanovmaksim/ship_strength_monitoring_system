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
    pub fn displacement_intensity(&self) -> Result<SpatiumFunctions, String> {
        let mut s_fs = vec![];
        let dw_i = self.deadweight_intensity.deadweight_intensity();
        let l_i = self.lightweight_intnesity.lightweight_intensity();
        for dwi_v in dw_i.as_ref() {
            let li_v = l_i.get(dwi_v.id()).unwrap();
            s_fs.push(li_v.add(dwi_v.clone())?);
        }
        Ok(SpatiumFunctions::new(s_fs))
    }
}
