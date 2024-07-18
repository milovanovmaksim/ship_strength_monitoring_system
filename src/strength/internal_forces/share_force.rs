use crate::strength::{

    load::total_shipload::TotalShipload,
    ship::{
        ship_dimensions::ShipDimensions, spatium_function::SpatiumFunction,
        spatium_functions::SpatiumFunctions,
    },
};

///
/// Перерезывающая (касательная) сила.
pub(crate) struct ShareForce<'a> {
    total_shipload: TotalShipload<'a>,
}

impl<'a> ShareForce<'a> {
    ///
    /// Основной конструктор.
    pub fn new(total_shipload: TotalShipload<'a>) -> Self {
        ShareForce { total_shipload }
    }

    ///
    /// Перерезывающая сила [т].
    pub fn share_force(
        &self,
        ship_dimensions: &ShipDimensions,
    ) -> Result<SpatiumFunctions, String> {
        let total_shipload = self.total_shipload.total_shipload(ship_dimensions)?;
        let mut share_force = total_shipload.integral_vul();
        let nose_share_force = share_force.last().unwrap().f_x2();
        let max_share_force = share_force.max().unwrap();
        if nose_share_force.abs() / max_share_force > 0.05 {
            warn!(
                "Эпюра перерезывающих сил не замкнута. Незамыкание эпюры: N(nose) / Nmax = {}",
                nose_share_force / max_share_force
            );
        }
        let mut f_x1 = 0.0;
        let mut s_fs = vec![];
        let mut x = ship_dimensions.length_spatium();
        let lbp = ship_dimensions.lbp();
        for s_f in share_force.into_iter() {
            let f_x2 = s_f.f_x2() - nose_share_force * x / lbp;
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
        share_force = SpatiumFunctions::new(s_fs);
        Ok(share_force)
    }
}
