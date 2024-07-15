use crate::strength::ship::spatium_function::SpatiumFunction;

use super::{
    load::total_shipload::TotalShipload,
    ship::{ship_dimensions::ShipDimensions, spatium_functions::SpatiumFunctions},
};

///
/// Перерезывающая (касательная) сила.
pub(crate) struct ShareForce<'a> {
    total_shipload: TotalShipload<'a>,
}

impl<'a> ShareForce<'a> {
    ///
    /// Основной конструктор.
    pub fn new(total_shipload: TotalShipload<'a>) -> Self {
        ShareForce { total_shipload }
    }

    ///
    /// Перерезывающая сила [т].
    pub fn share_force(
        &self,
        ship_dimensions: &ShipDimensions,
    ) -> Result<SpatiumFunctions, String> {
        let mut share_force = 0.0;
        let mut spatium_functions = vec![];
        let total_shipload = self.total_shipload.total_shipload(ship_dimensions)?;
        for s_f in total_shipload {
            let integral = s_f.integral();
            let share_force_s_f = SpatiumFunction::new(
                s_f.id(),
                s_f.x1(),
                s_f.x2(),
                share_force,
                share_force + integral,
            );
            share_force += integral;
            spatium_functions.push(share_force_s_f);
        }
        Ok(SpatiumFunctions::new(spatium_functions))
    }
}
