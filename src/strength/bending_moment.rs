use crate::strength::ship::spatium_function::SpatiumFunction;

use super::{
    share_force::ShareForce,
    ship::{ship_dimensions::ShipDimensions, spatium_functions::SpatiumFunctions},
};

///
/// Изгибающий момент.
pub(crate) struct BendingMoment<'a> {
    share_force: ShareForce<'a>,
}

impl<'a> BendingMoment<'a> {
    pub fn new(share_force: ShareForce<'a>) -> Self {
        BendingMoment { share_force }
    }

    ///
    /// Изгибающий момент [т*м].
    pub fn bending_moment(
        &self,
        ship_dimensions: &ShipDimensions,
    ) -> Result<SpatiumFunctions, String> {
        let share_force = self.share_force.share_force(ship_dimensions)?;
        Ok(share_force.integral_vul())
    }
}
