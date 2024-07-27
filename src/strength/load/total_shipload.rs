use std::rc::Rc;

use crate::strength::{
    buoyancy_intensity::buoyancy_intensity::BuoyancyIntensity,
    displacement::displacement_intensity::DisplacementIntensity,
    ship::{ship_dimensions::ShipDimensions, spatium_functions::SpatiumFunctions},
};

///
/// Интенсивность суммарной нагрузки по длине судна, действующей на корпус судна.
pub struct TotalShipload {
    disp_i: Rc<DisplacementIntensity>,
    b_i: Rc<BuoyancyIntensity>,
}

impl TotalShipload {
    ///
    /// Основной конструктор.
    pub fn new(disp_i: Rc<DisplacementIntensity>, b_i: Rc<BuoyancyIntensity>) -> Self {
        TotalShipload { disp_i, b_i }
    }

    ///
    /// Возвращает интенсивность суммарной нагрузки на корпус судна т/м.
    /// Интенсивность суммарной нагрузки определяется как алгебраическая сумма
    /// интенсивностей водоизмещения и сил поддержания по длине судна.
    pub fn total_shipload(
        &self,
        ship_dimensions: &ShipDimensions,
    ) -> Result<SpatiumFunctions, String> {
        let mut total_shipload = vec![];
        let disp_i_sfs = self.disp_i.displacement_intensity()?;
        let bi_sfs = self.b_i.buoyancy_intensity(ship_dimensions)?;
        for disp_i_sf in disp_i_sfs.as_ref() {
            let bi_sf = bi_sfs.get(disp_i_sf.id()).unwrap();
            total_shipload.push(bi_sf.add(disp_i_sf.clone())?);
        }
        Ok(SpatiumFunctions::new(total_shipload))
    }
}
