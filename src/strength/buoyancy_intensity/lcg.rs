use crate::strength::displacement::displacement_intensity::DisplacementIntensity;

///
/// Абcцисса центр тяжести судна. Англ.: Longitudinal Center of Gravity (LCG).
/// Отсчитывается от мидель шпангоута. Имеет положительный знак от мидель шпангоута в нос судна.
#[derive(Clone, Copy)]
pub struct LCG {
    lcg: f64,
}

impl LCG {
    pub fn new(lcg: f64) -> Self {
        LCG { lcg }
    }

    pub fn from_disp_i(disp_i: &DisplacementIntensity) -> LCG {
        let displacement_intensity = disp_i.displacement_intensity();
        let mut moment = 0.0;
        let mut displacement_tonnage = 0.0;
        for spatium in displacement_intensity.as_ref() {
            let integral = spatium.integral();
            displacement_tonnage += integral;
            moment += integral * spatium.abscissa();
        }
        LCG::new(moment / displacement_tonnage)
    }

    pub fn lcg(&self) -> f64 {
        self.lcg
    }
}
