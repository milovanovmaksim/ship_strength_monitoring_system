use crate::strength::displacement::displacement_intensity::DisplacementIntensity;

///
/// Абcцисса центр тяжести судна. Англ.: Longitudinal Center of Gravity (LCG).
/// Отсчитывается от мидель шпангоута. Имеет положительный знак от мидель шпангоута в нос судна.
pub struct LCG {
    displacement_intensity: DisplacementIntensity,
}

impl LCG {
    pub fn new(displacement_intensity: DisplacementIntensity) -> Self {
        LCG {
            displacement_intensity,
        }
    }

    pub fn lcg(&self) -> f64 {
        let displacement_intensity = self.displacement_intensity.spatium_functions();
        let mut moment = 0.0;
        let mut displacement = 0.0;
        for spatium in displacement_intensity.as_ref() {
            displacement += spatium.integral();
            moment += spatium.integral() * spatium.abscissa();
        }
        moment / displacement
    }
}
