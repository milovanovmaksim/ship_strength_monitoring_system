use std::ops::Div;

#[derive(Debug, Clone, Copy)]
pub(crate) struct WaterDensity {
    water_density: f64,
}

impl WaterDensity {
    pub fn new(water_density: f64) -> Self {
        WaterDensity { water_density }
    }

    pub fn water_density(&self) -> f64 {
        self.water_density
    }
}

impl Div<WaterDensity> for f64 {
    // The division of rational numbers is a closed operation.
    type Output = f64;

    fn div(self, rhs: WaterDensity) -> Self::Output {
        if rhs.water_density() < 0.0 {
            panic!("Деление на ноль.")
        }
        self / rhs.water_density()
    }
}
