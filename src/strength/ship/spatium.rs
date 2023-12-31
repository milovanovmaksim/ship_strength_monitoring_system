
#[derive(Debug)]
pub struct Spatium {
    id: i64,
    x1: f64,
    x2: f64,
    f_x1: f64,
    f_x2: f64,

}

impl Spatium {
    pub fn new(id: i64, x1: f64,
        x2: f64,
        f_x1: f64,
        f_x2: f64,) -> Self {
        Spatium { id, x1, x2, f_x1, f_x2 }
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

    pub fn id(&self) -> i64 { self.id }
}