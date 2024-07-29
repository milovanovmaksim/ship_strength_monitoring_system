use crate::strength::{
    load::total_shipload::TotalShipload,
    ship::{ship_dimensions::ShipDimensions, spatium_functions::SpatiumFunctions},
};

use super::with_correction;

///
/// Перерезывающая (касательная) сила.
pub struct ShareForce {
    share_force_: SpatiumFunctions,
    share_force_with_correction_: Option<SpatiumFunctions>,
}

impl ShareForce {
    ///
    /// Основной конструктор.
    pub fn new(share_force: SpatiumFunctions) -> Self {
        ShareForce {
            share_force_: share_force,
            share_force_with_correction_: None,
        }
    }

    pub fn from_total_ship_load(total_shipload: &TotalShipload) -> ShareForce {
        let share_force = total_shipload.total_shipload().integral_vul();
        ShareForce::new(share_force)
    }

    pub fn with_correction(mut self, ship_dimensions: ShipDimensions) -> ShareForce {
        self.share_force_with_correction_ =
            Some(with_correction(&self.share_force_, ship_dimensions));
        self
    }
    pub fn share_force(&self) -> &SpatiumFunctions {
        &self.share_force_
    }

    pub fn share_force_with_correction(&self) -> Option<&SpatiumFunctions> {
        self.share_force_with_correction_.as_ref()
    }
}
