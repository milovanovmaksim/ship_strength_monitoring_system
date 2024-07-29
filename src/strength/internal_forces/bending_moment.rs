use super::{share_force::ShareForce, with_correction};
use crate::strength::ship::{ship_dimensions::ShipDimensions, spatium_functions::SpatiumFunctions};

///
/// Изгибающий момент.
pub struct BendingMoment {
    bending_moment_: SpatiumFunctions,
    bending_moment_with_correction_: Option<SpatiumFunctions>,
}

impl BendingMoment {
    ///
    /// Основной конструктор.
    pub fn new(bending_moment: SpatiumFunctions) -> Self {
        BendingMoment {
            bending_moment_: bending_moment,
            bending_moment_with_correction_: None,
        }
    }

    pub fn from_share_force(share_force: &ShareForce) -> BendingMoment {
        BendingMoment::new(share_force.share_force().integral_vul())
    }

    pub fn with_correction(mut self, ship_dimensions: ShipDimensions) -> BendingMoment {
        self.bending_moment_with_correction_ =
            Some(with_correction(&self.bending_moment_, ship_dimensions));
        self
    }

    pub fn bending_momant(&self) -> &SpatiumFunctions {
        &self.bending_moment_
    }

    pub fn bending_moment_with_correction(&self) -> &Option<SpatiumFunctions> {
        &self.bending_moment_with_correction_
    }
}
