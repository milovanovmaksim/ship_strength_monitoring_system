use crate::strength::{
    buoyancy_intensity::buoyancy_intensity::BuoyancyIntensity,
    displacement::displacement_intensity::DisplacementIntensity,
    ship::spatium_functions::SpatiumFunctions,
};

///
/// Интенсивность суммарной нагрузки по длине судна, действующей на корпус судна.
pub struct TotalShipload {
    total_shipload_: SpatiumFunctions,
}

impl TotalShipload {
    ///
    /// Основной конструктор.
    pub fn new(total_shipload: SpatiumFunctions) -> Self {
        TotalShipload {
            total_shipload_: total_shipload,
        }
    }

    pub fn from_disp_i_and_b_i(
        disp_i: &DisplacementIntensity,
        b_i: &BuoyancyIntensity,
    ) -> Result<TotalShipload, String> {
        let mut total_shipload = vec![];
        let disp_i_sfs = disp_i.displacement_intensity();
        let bi_sfs = b_i.buoyancy_intensity();
        for disp_i_sf in disp_i_sfs.as_ref() {
            let bi_sf = bi_sfs.get(disp_i_sf.id()).unwrap();
            total_shipload.push(bi_sf.add(disp_i_sf.clone())?);
        }
        Ok(TotalShipload::new(SpatiumFunctions::new(total_shipload)))
    }

    ///
    /// Возвращает интенсивность суммарной нагрузки на корпус судна т/м.
    /// Интенсивность суммарной нагрузки определяется как алгебраическая сумма
    /// интенсивностей водоизмещения и сил поддержания по длине судна.
    pub fn total_shipload(&self) -> &SpatiumFunctions {
        &self.total_shipload_
    }
}
