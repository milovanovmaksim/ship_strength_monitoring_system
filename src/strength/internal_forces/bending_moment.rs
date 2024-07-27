use super::{share_force::ShareForce, with_correction};
use crate::strength::ship::{
    ship_dimensions::ShipDimensions, spatium_function::SpatiumFunction,
    spatium_functions::SpatiumFunctions,
};

///
/// Изгибающий момент.
pub struct BendingMoment {
    bending_moment_: SpatiumFunctions,
}

impl BendingMoment {
    ///
    /// Основной конструктор.
    pub fn new(bending_moment: SpatiumFunctions) -> Self {
        BendingMoment {
            bending_moment_: bending_moment,
        }
    }

    pub fn from_share_force(share_force: &ShareForce) -> BendingMoment {
        BendingMoment::new(share_force.share_force().integral_vul())
    }

    pub fn with_correction(self, ship_dimensions: ShipDimensions) -> BendingMoment {
        BendingMoment::new(with_correction(&self.bending_moment_, ship_dimensions))
    }

    pub fn bending_momant(&self) -> &SpatiumFunctions {
        &self.bending_moment_
    }
}
