use super::{internal_force::InternalForce, share_force::ShareForce};
use crate::strength::ship::{ship_dimensions::ShipDimensions, spatium_functions::SpatiumFunctions};

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
    ///
    /// Возвращает подинтегральную функцию изгибающего момента, т.е перерезывающую силу.
    fn integrand(&self, ship_dimensions: &ShipDimensions) -> Result<SpatiumFunctions, String> {
        self.share_force.internal_force(ship_dimensions)
    }
}
