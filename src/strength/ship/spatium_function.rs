use super::ship_dimensions::ShipDimensions;


#[derive(Debug)]
pub struct SpatiumFunction {
    x1: f64,
    x2: f64,
    f_x1: f64,
    f_x2: f64,

}

impl SpatiumFunction {
    pub fn new(x1: f64,
        x2: f64,
        f_x1: f64,
        f_x2: f64,) -> Self {
        SpatiumFunction { x1, x2, f_x1, f_x2 }
    }

    pub fn from_ship_dimensions(id: u64, ship_demensions: &ShipDimensions, f_x1: f64, f_x2: f64) -> Self {
        let spatium_start_coordinate = ship_demensions.spatium_start_coordinate(id);
        let spatium_end_coordinate = ship_demensions.spatium_end_coordinate(id);
        SpatiumFunction::new(spatium_start_coordinate, spatium_end_coordinate, f_x1, f_x2)

    }

    pub fn integral(&self) -> f64 {
        ((self.f_x1 + self.f_x2) / 2.0) * (self.x2 - self.x1)
    }

    pub fn x1(&self) -> f64 {
        self.x1
    }

    pub fn x2(&self) -> f64 {
        self.x2
    }

    pub fn f_x1(&self) -> f64 {
        self.f_x1
    }

    pub fn f_x2(&self) -> f64 {
        self.f_x2
    }

}