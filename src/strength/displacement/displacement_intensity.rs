use crate::strength::{
    deadweight::deadweight_intensity::DeadweightIntensity,
    lightweight::lightweight_intensity::LightweightIntensity,
    ship::{ship_dimensions::ShipDimensions, spatium_functions::SpatiumFunctions},
};

///
/// Интенсивность водоизмещения судна по его длине.
pub struct DisplacementIntensity<'a> {
    dw_i: &'a DeadweightIntensity<'a>,
    lw_i: &'a LightweightIntensity,
}

impl<'a> DisplacementIntensity<'a> {
    ///
    /// Основной конструктор.
    pub fn new(dw_i: &'a DeadweightIntensity<'a>, lw_i: &'a LightweightIntensity) -> Self {
        DisplacementIntensity { dw_i, lw_i }
    }

    ///
    /// Возвращает интенсивность водоизмещения судна по его длине т/м.
    /// Интенсивность водоизмещения определяется как алгебраическая сумма
    /// интенсивностей дедвейта и массы корпуса судна по его длине.
    pub fn displacement_intensity(
        &self,
        ship_dimensions: &ShipDimensions,
    ) -> Result<SpatiumFunctions, String> {
        let mut s_fs = vec![];
        let dw_i = self.dw_i.deadweight_intensity(ship_dimensions);
        let l_i = self.lw_i.lightweight_intensity();
        for dwi_v in dw_i.as_ref() {
            let li_v = l_i.get(dwi_v.id()).unwrap();
            s_fs.push(li_v.add(dwi_v.clone())?);
        }
        Ok(SpatiumFunctions::new(s_fs))
    }
}
