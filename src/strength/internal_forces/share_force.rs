use std::rc::Rc;

use super::internal_force::InternalForce;
use crate::strength::{
    load::total_shipload::TotalShipload,
    ship::{ship_dimensions::ShipDimensions, spatium_functions::SpatiumFunctions},
};

///
/// Перерезывающая (касательная) сила.
pub(crate) struct ShareForce {
    total_shipload: Rc<TotalShipload>,
}

impl ShareForce {
    ///
    /// Основной конструктор.
    pub fn new(total_shipload: Rc<TotalShipload>) -> Self {
        ShareForce { total_shipload }
    }
}

impl InternalForce for ShareForce {
    ///
    /// Возвращает подинтегральную функцию перерезывающей силы,
    /// т.е распределение интенсивности суммарной нагрузки, действующей на корпус судна.
    fn integrand(&self, ship_dimensions: &ShipDimensions) -> Result<SpatiumFunctions, String> {
        self.total_shipload.total_shipload(ship_dimensions)
    }
}
