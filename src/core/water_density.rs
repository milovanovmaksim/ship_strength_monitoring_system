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
