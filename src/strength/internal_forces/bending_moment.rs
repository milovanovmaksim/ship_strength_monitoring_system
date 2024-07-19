use log::warn;

use crate::strength::ship::{
    ship_dimensions::ShipDimensions, spatium_function::SpatiumFunction,
    spatium_functions::SpatiumFunctions,
};

use super::share_force::ShareForce;

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

    ///
    /// Изгибающий момент [т*м].
    /// Изгибающий момент определяется путем интегрирования эпюры поперечных сил.
    pub fn bending_moment(
        &self,
        ship_dimensions: &ShipDimensions,
    ) -> Result<SpatiumFunctions, String> {
        let share_force = self.share_force.share_force(ship_dimensions)?;
        let bending_moment = share_force.integral_vul();
        let nose_moment = bending_moment.last().unwrap().f_x2();
        let max_moment = bending_moment.max().unwrap();
        if nose_moment.abs() / max_moment > 0.05 {
            warn!(
                "Эпюра изгибающих моментов не замкнута. Незамыкание эпюры: M(nose) / Mmax = {}",
                nose_moment / max_moment
            );
        }
        Ok(SpatiumFunctions::with_correction(
            bending_moment,
            ship_dimensions,
        ))
    }
}
