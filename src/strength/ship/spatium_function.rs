use crate::core::round::Round;

use super::ship_dimensions::ShipDimensions;

///
/// Contains the results of any calculations for the spatium.
#[derive(Debug, PartialEq)]
pub struct SpatiumFunction {
    id: u64,
    x1: f64,
    x2: f64,
    f_x1: f64,
    f_x2: f64,

}

impl SpatiumFunction {
    ///
    /// Create new object.
    pub fn new(id: u64, x1: f64,
        x2: f64,
        f_x1: f64,
        f_x2: f64,) -> Self {
        SpatiumFunction { id, x1, x2, f_x1, f_x2 }
    }


    ///
    /// Create new object by id.
    pub fn from_id(id: i64, ship_demensions: &ShipDimensions, f_x1: f64, f_x2: f64) -> Self {
        let spatium_start_coordinate = ship_demensions.spatium_start_coordinate(id).my_round(2);
        let spatium_end_coordinate = ship_demensions.spatium_end_coordinate(id).my_round(2);
        SpatiumFunction::new(id as u64, spatium_start_coordinate, spatium_end_coordinate, f_x1.my_round(2), f_x2.my_round(2))

    }

    /// Determines the integral of the function using a numerical method(trapezoidal method).
    pub fn integral(&self) -> f64 {
        ((self.f_x1 + self.f_x2) / 2.0) * (self.x2 - self.x1)
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

    /// Add curretn object with another.
    pub fn add(&self, term: SpatiumFunction) -> SpatiumFunction {
        let f_x1 = self.f_x1 + term.f_x1();
        let f_x2 = self.f_x2 + term.f_x2();
        SpatiumFunction::new(self.id, self.x1, self.x2, f_x1, f_x2)
    }

}