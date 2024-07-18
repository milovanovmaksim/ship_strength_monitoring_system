use serde::Deserialize;

use crate::core::round::Round;

use super::ship_dimensions::ShipDimensions;

///
/// Содержит результаты расчета
/// (интенсивность водоизмещение, перерезывающая сила, изгибающий момент и.т.д) для одной теоретической шпации судна.
#[derive(Debug, PartialEq, Copy, Clone, Deserialize)]
pub struct SpatiumFunction {
    id: u64,
    x1: f64,
    x2: f64,
    f_x1: f64,
    f_x2: f64,
}

impl SpatiumFunction {
    ///
    /// Основной конструктор.
    pub fn new(id: u64, x1: f64, x2: f64, f_x1: f64, f_x2: f64) -> Self {
        SpatiumFunction {
            id,
            x1,
            x2,
            f_x1,
            f_x2,
        }
    }

    ///
    /// Create new object by id.
    pub fn from_id(id: u64, ship_dimensions: &ShipDimensions, f_x1: f64, f_x2: f64) -> Self {
        let spatium_start_coordinate = ship_dimensions.spatium_start_coordinate(id).my_round(2);
        let spatium_end_coordinate = ship_dimensions.spatium_end_coordinate(id).my_round(2);
        SpatiumFunction::new(
            id,
            spatium_start_coordinate,
            spatium_end_coordinate,
            f_x1,
            f_x2,
        )
    }

    /// Compute the integral of the spatium function using a numerical method(trapezoidal method).
    pub fn integral(&self) -> f64 {
        ((self.f_x1 + self.f_x2) / 2.0) * (self.x2 - self.x1)
    }

    pub fn abscissa(&self) -> f64 {
        let delta = self.x2 - self.x1;
        self.x1 + delta / 2.0
    }

    ///
    /// Return id of the object.
    pub fn id(&self) -> u64 {
        self.id
    }

    ///
    /// Spatium start coordinate.
    pub fn x1(&self) -> f64 {
        self.x1
    }

    ///
    /// Spatium end coordinate.
    pub fn x2(&self) -> f64 {
        self.x2
    }

    ///
    /// Return function value at a point x1.
    pub fn f_x1(&self) -> f64 {
        self.f_x1
    }

    ///
    /// Return function value at a point x2.
    pub fn f_x2(&self) -> f64 {
        self.f_x2
    }

    /// Сложение шпаций с одинаковыми id.
    /// Возвращает новый объект.
    pub fn add(&self, term: SpatiumFunction) -> Result<SpatiumFunction, String> {
        if term.id == self.id {
            let f_x1 = self.f_x1 + term.f_x1();
            let f_x2 = self.f_x2 + term.f_x2();
            return Ok(SpatiumFunction::new(self.id, self.x1, self.x2, f_x1, f_x2));
        }
        Err("Сложение шпаций с разными id".to_string())
    }
}
