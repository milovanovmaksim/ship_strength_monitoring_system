use crate::strength::{
    load::total_shipload::TotalShipload,
    ship::{
        ship_dimensions::ShipDimensions, spatium_function::SpatiumFunction,
        spatium_functions::SpatiumFunctions,
    },
};

use super::closed_diagram::InternalForce;

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
}

impl<'a> InternalForce for ShareForce<'a> {
    fn integrand(&self, ship_dimensions: &ShipDimensions) -> Result<SpatiumFunctions, String> {
        self.total_shipload.total_shipload(ship_dimensions)
    }
}
