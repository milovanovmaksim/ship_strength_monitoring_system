use tracing::instrument;

use crate::strength::{
    deadweight::deadweight_intensity::DeadweightIntensity,
    lightweight::lightweight_intensity::LightweightIntensity,
    ship::spatium_functions::SpatiumFunctions,
};

///
/// Интенсивность водоизмещения судна по его длине.
pub struct DisplacementIntensity {
    disp_i: SpatiumFunctions,
}

impl DisplacementIntensity {
    ///
    /// Основной конструктор.
    pub fn new(disp_i: SpatiumFunctions) -> Self {
        DisplacementIntensity { disp_i }
    }

    ///
    /// Вспомогательный конструктор.
    #[instrument(skip_all, err, target = "DisplacementIntensity::from_dw_i_and_lw_i")]
    pub fn from_dw_i_and_lw_i(
        dw_i: &DeadweightIntensity,
        lw_i: &LightweightIntensity,
    ) -> Result<DisplacementIntensity, String> {
        let mut s_fs = vec![];
        let dw_i = dw_i.deadweight_intensity();
        let l_i = lw_i.lightweight_intensity();
        for dwi_v in dw_i.as_ref() {
            let li_v = l_i.get(dwi_v.id()).unwrap();
            s_fs.push(li_v.add(dwi_v.clone())?);
        }
        Ok(DisplacementIntensity::new(SpatiumFunctions::new(s_fs)))
    }

    ///
    /// Возвращает интенсивность водоизмещения судна по его длине т/м.
    /// Интенсивность водоизмещения определяется как алгебраическая сумма
    /// интенсивностей дедвейта и массы корпуса судна по его длине.
    pub fn displacement_intensity(&self) -> &SpatiumFunctions {
        &self.disp_i
    }
}
