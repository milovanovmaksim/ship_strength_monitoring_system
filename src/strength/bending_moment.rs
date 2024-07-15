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
        let mut spatium_functions = vec![];
        let mut bending_moment = 0.0;
        for s_f in self.share_force.share_force(ship_dimensions)? {
            let integral = s_f.integral();
            let spatium_function = SpatiumFunction::new(
                s_f.id(),
                s_f.x1(),
                s_f.x2(),
                bending_moment,
                bending_moment + integral,
            );
            bending_moment += integral;
            spatium_functions.push(spatium_function);
        }
        Ok(SpatiumFunctions::new(spatium_functions))
    }
}
