use crate::strength::ship::spatium_function::SpatiumFunction;

use super::ship::{ship_dimensions::ShipDimensions, spatium_functions::SpatiumFunctions};

pub mod bending_moment;
pub mod share_force;

///
/// Разносит невязки линейно по шпациям.
/// Так как удифферентовка судна осуществляется приближенно,
/// то после интегрирования суммарной нагрузки в носовом шпангоуте появляются
/// невязки в эпюрах: N(nose) != 0, M(nose) != 0. Для получения эпюр N(x), M(x),
/// удовлетворяющих граничным условиям, данные невязки линейно разносят по шпациям.
/// Исправленные N и M равны:
/// N_исп = N(x) - N_nose * x / L, M_исп = M_nose * x / L, где
///     x - отсчитывается от кормового перепендикуляра,
///     N(x) - перерезывающая сила в точке с координатой x,
///     M(x) - изгибающий момент с координатой x,
///     N_nose - перерезывающая сила в носовом шпангоуте,
///     M_nose - изгибающий момент в носовом шпангоуте,
///     L - длина между перпендикулярами.
fn with_correction(
    spatium_functions: &SpatiumFunctions,
    ship_dimensions: ShipDimensions,
) -> SpatiumFunctions {
    let nose_value = spatium_functions.last().unwrap().f_x2();
    let mut f_x1 = 0.0;
    let mut internal_force_with_correct = vec![];
    let mut x = ship_dimensions.length_spatium();
    let lbp = ship_dimensions.lbp();
    for s_f in spatium_functions.as_ref() {
        let f_x2 = s_f.f_x2() - nose_value * x / lbp;
        internal_force_with_correct.push(SpatiumFunction::new(
            s_f.id(),
            s_f.x1(),
            s_f.x2(),
            f_x1,
            f_x2,
        ));
        f_x1 = f_x2;
        x += ship_dimensions.length_spatium();
    }
    SpatiumFunctions::new(internal_force_with_correct)
}
