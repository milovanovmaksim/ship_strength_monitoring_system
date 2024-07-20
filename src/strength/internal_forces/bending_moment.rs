use crate::strength::ship::{ship_dimensions::ShipDimensions, spatium_functions::SpatiumFunctions};

use super::{closed_diagram::InternalForce, share_force::ShareForce};

///
/// Изгибающий момент.
pub(crate) struct BendingMoment<'a> {
    share_force: ShareForce<'a>,
}

impl<'a> BendingMoment<'a> {
    ///
    /// Основной конструктор.
    pub fn new(share_force: ShareForce<'a>) -> Self {
        BendingMoment { share_force }
    }
}

impl<'a> InternalForce for BendingMoment<'a> {
    fn integrand(&self, ship_dimensions: &ShipDimensions) -> Result<SpatiumFunctions, String> {
        self.share_force.internal_force(ship_dimensions)
    }
}
