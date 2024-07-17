use crate::strength::{
    load::total_shipload::TotalShipload,
    ship::{
        ship_dimensions::ShipDimensions, spatium_function::SpatiumFunction,
        spatium_functions::SpatiumFunctions,
    },
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
        let total_shipload = self.total_shipload.total_shipload(ship_dimensions)?;
        Ok(total_shipload.integral_vul())
    }
}
