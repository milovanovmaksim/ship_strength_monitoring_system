use log::{info, warn};

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
    ///
    /// Основной конструктор.
    pub fn new(share_force: ShareForce<'a>) -> Self {
        BendingMoment { share_force }
    }

    ///
    /// Изгибающий момент [т*м].
    /// Изгибающий момент определяется путем интегрирования эпюры поперечных.
    pub fn bending_moment(
        &self,
        ship_dimensions: &ShipDimensions,
    ) -> Result<SpatiumFunctions, String> {
        // TODO: проверить "незамыкание" эпюры, внести поправку к эпюре в свззи с "незамыканием".
        let share_force = self.share_force.share_force(ship_dimensions)?;
        let bending_moment = share_force.integral_vul();
        let last_value = bending_moment.last().unwrap().f_x2().abs();
        let max_value = bending_moment.max();
        if last_value / max_value > 0.05 {
            warn!(
                "Эпюра изгибающих моментов не замкнута. Незамыкание эпюры: M(20) / Mmax = {}",
                last_value / max_value
            );
        }
        Ok(bending_moment)
    }
}
