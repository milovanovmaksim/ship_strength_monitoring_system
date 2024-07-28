use crate::strength::{
    load::total_shipload::TotalShipload,
    ship::{ship_dimensions::ShipDimensions, spatium_functions::SpatiumFunctions},
};

use super::with_correction;

///
/// Перерезывающая (касательная) сила.
pub struct ShareForce {
    share_force_: SpatiumFunctions,
}

impl ShareForce {
    ///
    /// Основной конструктор.
    pub fn new(share_force: SpatiumFunctions) -> Self {
        ShareForce {
            share_force_: share_force,
        }
    }

    pub fn from_total_ship_load(total_shipload: &TotalShipload) -> ShareForce {
        let share_force = total_shipload.total_shipload().integral_vul();
        ShareForce::new(share_force)
    }

    pub fn with_correction(self, ship_dimensions: ShipDimensions) -> ShareForce {
        ShareForce::new(with_correction(&self.share_force_, ship_dimensions))
    }
    pub fn share_force(&self) -> &SpatiumFunctions {
        &self.share_force_
    }
}
