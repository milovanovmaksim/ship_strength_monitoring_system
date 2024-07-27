use std::rc::Rc;

use super::{internal_force::InternalForce, share_force::ShareForce};
use crate::strength::ship::{ship_dimensions::ShipDimensions, spatium_functions::SpatiumFunctions};

///
/// Изгибающий момент.
pub struct BendingMoment {
    share_force: Rc<ShareForce>,
}

impl BendingMoment {
    ///
    /// Основной конструктор.
    pub fn new(share_force: Rc<ShareForce>) -> Self {
        BendingMoment { share_force }
    }
}

impl InternalForce for BendingMoment {
    ///
    /// Возвращает подинтегральную функцию изгибающего момента,
    /// т.е распределение перерезывающей силы по длине корпуса судна.
    fn integrand(&self, ship_dimensions: &ShipDimensions) -> Result<SpatiumFunctions, String> {
        self.share_force.internal_force(ship_dimensions)
    }
}
