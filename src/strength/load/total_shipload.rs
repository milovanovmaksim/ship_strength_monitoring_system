use crate::strength::{
    buoyancy_intensity::buoyancy_intensity::BuoyancyIntensity,
    displacement::displacement_intensity::DisplacementIntensity,
    ship::{ship_dimensions::ShipDimensions, spatium_functions::SpatiumFunctions},
};

///
/// Интенсивность суммарной нагрузки по длине судна, действующей на корпус судна.
pub(crate) struct TotalShipload<'a> {
    d_i: DisplacementIntensity<'a>,
    b_i: BuoyancyIntensity<'a>,
}

impl<'a> TotalShipload<'a> {
    ///
    /// Основной конструктор.
    pub fn new(d_i: DisplacementIntensity<'a>, b_i: BuoyancyIntensity<'a>) -> Self {
        TotalShipload { d_i, b_i }
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
        let di_value = self.d_i.spatium_functions();
        let bi_value = self.b_i.buoyancy_intensity(ship_dimensions)?;
        for i in 0..ship_dimensions.number_spatiums() {
            let sf_1 = di_value.get(i).unwrap();
            let sf_2 = bi_value.get(i).unwrap();
            total_shipload.push(sf_1.add(sf_2.clone()));
        }
        Ok(SpatiumFunctions::new(total_shipload))
    }
}
