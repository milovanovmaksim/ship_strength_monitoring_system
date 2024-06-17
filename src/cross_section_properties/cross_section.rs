use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CrossSection {
    pub id: i32,
    I_z: f64,
    I_y: f64,
    z_up: f64,
    z_down: f64,
}

impl CrossSection {
    pub fn new(id: i32, I_z: f64, I_y: f64, z_up: f64, z_down: f64) -> Self {
        CrossSection {
            id,
            I_z,
            I_y,
            z_up,
            z_down,
        }
    }
}
