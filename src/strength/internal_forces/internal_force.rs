use crate::strength::ship::{
    ship_dimensions::ShipDimensions, spatium_function::SpatiumFunction,
    spatium_functions::SpatiumFunctions,
};

///
/// Замкнутая эпюра внутренних сил, т.е значения внутренних
/// сил в носовом и кормовом шпангоутах равны нулю.
pub trait InternalForce {
    ///
    /// Так как удифферентовка судна осуществляется приближенно,
    /// то после интегрирования суммарной нагрузки в носовом шпангоуте появляются
    /// невязки в эпюрах: N(nose) != 0, M(nose) != 0. Для получения эпюр N(x), M(x),
    /// удовлетворяющих граничным условиям, данные невязки линейно разносят по шпациям.
    /// Исправленные N и M равны:
    /// N_исп = N(x) - N_nose * x / L, M_исп = M_nose * x / L, где
    /// x - отсчитывается от кормового перепендикуляра,
    /// N(x) - перерезывающая сила в точке с координатой x,
    /// M(x) - изгибающий момент с координатой x,
    /// N_nose - перерезывающая сила в носовом шпангоуте,
    /// M_nose - изгибающий момент в носовом шпангоуте,
    /// L - длина между перпендикулярами.
    fn internal_force(&self, ship_dimensions: &ShipDimensions) -> Result<SpatiumFunctions, String> {
        let s_fs = self.integrand(ship_dimensions)?.integral_vul();
        let nose_value = s_fs.last().unwrap().f_x2();
        let mut f_x1 = 0.0;
        let mut s_fs_with_correction = vec![];
        let mut x = ship_dimensions.length_spatium();
        let lbp = ship_dimensions.lbp();
        for s_f in s_fs.into_iter() {
            let f_x2 = s_f.f_x2() - nose_value * x / lbp;
            s_fs_with_correction.push(SpatiumFunction::new(
                s_f.id(),
                s_f.x1(),
                s_f.x2(),
                f_x1,
                f_x2,
            ));
            f_x1 = f_x2;
            x += ship_dimensions.length_spatium();
        }
        Ok(SpatiumFunctions::new(s_fs_with_correction))
    }

    ///
    /// Возвращает подинтегральную функцию внутренней силы.
    fn integrand(&self, ship_dimensions: &ShipDimensions) -> Result<SpatiumFunctions, String>;
}
