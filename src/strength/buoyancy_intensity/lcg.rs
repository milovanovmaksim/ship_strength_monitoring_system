use std::{borrow::BorrowMut, cell::RefCell};

use crate::strength::displacement::displacement_intensity::DisplacementIntensity;

///
/// Абcцисса центр тяжести судна. Англ.: Longitudinal Center of Gravity (LCG).
/// Отсчитывается от мидель шпангоута. Имеет положительный знак от мидель шпангоута в нос судна.
pub struct LCG<'a> {
    displacement_intensity: DisplacementIntensity<'a>,
    lcg_v: RefCell<Option<f64>>,
}

impl<'a> LCG<'a> {
    pub fn new(displacement_intensity: DisplacementIntensity<'a>) -> Self {
        LCG {
            displacement_intensity,
            lcg_v: RefCell::new(None),
        }
    }

    pub fn lcg(&self) -> Result<f64, String> {
        let mut lcg_v = self.lcg_v.borrow_mut();
        if let Some(v) = *lcg_v {
            return Ok(v);
        } else {
            let displacement_intensity = self.displacement_intensity.displacement_intensity()?;
            let mut moment = 0.0;
            let mut displacement_tonnage = 0.0;
            for spatium in displacement_intensity.as_ref() {
                let integral = spatium.integral();
                displacement_tonnage += integral;
                moment += integral * spatium.abscissa();
            }
            *lcg_v = Some(moment / displacement_tonnage);
            Ok(moment / displacement_tonnage)
        }
    }
}
