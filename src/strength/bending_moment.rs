use log::warn;

use super::{
    share_force::ShareForce,
    ship::{
        ship_dimensions::ShipDimensions, spatium_function::SpatiumFunction,
        spatium_functions::SpatiumFunctions,
    },
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
    /// Изгибающий момент определяется путем интегрирования эпюры поперечных сил.
    pub fn bending_moment(
        &self,
        ship_dimensions: &ShipDimensions,
    ) -> Result<SpatiumFunctions, String> {
        let lbp = ship_dimensions.lbp();
        let share_force = self.share_force.share_force(ship_dimensions)?;
        let mut bending_moment = share_force.integral_vul();
        let nose_moment = bending_moment.last().unwrap().f_x2();
        let max_moment = bending_moment.max().unwrap();
        if nose_moment.abs() / max_moment > 0.05 {
            warn!(
                "Эпюра изгибающих моментов не замкнута. Незамыкание эпюры: M(nose) / Mmax = {}",
                nose_moment / max_moment
            );
        }
        let mut f_x1 = 0.0;
        let mut s_fs = vec![];
        let mut x = ship_dimensions.length_spatium();
        for s_f in bending_moment.into_iter() {
            let f_x2 = s_f.f_x2() - nose_moment * x / lbp;
            s_fs.push(SpatiumFunction::new(
                s_f.id(),
                s_f.x1(),
                s_f.x2(),
                f_x1,
                f_x2,
            ));
            f_x1 = f_x2;
            x += ship_dimensions.length_spatium();
        }
        bending_moment = SpatiumFunctions::new(s_fs);
        Ok(bending_moment)
    }
}
