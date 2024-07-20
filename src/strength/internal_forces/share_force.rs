use super::internal_force::InternalForce;
use crate::strength::{
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
}

impl<'a> InternalForce for ShareForce<'a> {
    ///
    /// Возвращает подинтегральную функцию перерезывающей силы,
    /// т.е распределение интенсивности суммарной нагрузки, действующей на корпус судна.
    fn integrand(&self, ship_dimensions: &ShipDimensions) -> Result<SpatiumFunctions, String> {
        self.total_shipload.total_shipload(ship_dimensions)
    }
}
