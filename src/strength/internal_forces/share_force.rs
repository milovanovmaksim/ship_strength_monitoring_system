use crate::strength::{
    load::total_shipload::TotalShipload,
    ship::{
        ship_dimensions::ShipDimensions, spatium_function::SpatiumFunction,
        spatium_functions::SpatiumFunctions,
    },
};
use log::warn;

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
        let total_shipload = self.total_shipload.total_shipload(ship_dimensions)?;
        let share_force = total_shipload.integral_vul();
        let nose_share_force = share_force.last().unwrap().f_x2();
        let max_share_force = share_force.max().unwrap();
        if nose_share_force.abs() / max_share_force > 0.05 {
            warn!(
                "Эпюра перерезывающих сил не замкнута. Незамыкание эпюры: N(nose) / Nmax = {}",
                nose_share_force / max_share_force
            );
        }
        Ok(share_force)
    }
}
