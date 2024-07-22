use std::rc::Rc;

use crate::strength::{
    displacement::displacement_intensity::DisplacementIntensity,
    ship::ship_dimensions::ShipDimensions,
};

///
/// Абcцисса центр тяжести судна. Англ.: Longitudinal Center of Gravity (LCG).
/// Отсчитывается от мидель шпангоута. Имеет положительный знак от мидель шпангоута в нос судна.
pub struct LCG {
    displacement_intensity: Rc<DisplacementIntensity>,
    ship_dimensions: ShipDimensions,
}

impl LCG {
    pub fn new(
        displacement_intensity: Rc<DisplacementIntensity>,
        ship_dimensions: ShipDimensions,
    ) -> Self {
        LCG {
            displacement_intensity,
            ship_dimensions,
        }
    }

    pub fn lcg(&self) -> Result<f64, String> {
        let displacement_intensity = self.displacement_intensity.displacement_intensity()?;
        let mut moment = 0.0;
        let mut displacement_tonnage = 0.0;
        for spatium in displacement_intensity.as_ref() {
            let integral = spatium.integral();
            displacement_tonnage += integral;
            moment += integral * spatium.abscissa();
        }
        Ok(moment / displacement_tonnage)
    }
}
