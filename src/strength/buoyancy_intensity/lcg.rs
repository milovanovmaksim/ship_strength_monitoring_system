use crate::strength::displacement::displacement_intensity::DisplacementIntensity;

///
/// Абcцисса центр тяжести судна. Англ.: Longitudinal Center of Gravity (LCG).
/// Отсчитывается от мидель шпангоута. Имеет положительный знак от мидель шпангоута в нос судна.
pub struct LCG<'a> {
    displacement_intensity: DisplacementIntensity<'a>,
}

impl<'a> LCG<'a> {
    pub fn new(displacement_intensity: DisplacementIntensity<'a>) -> Self {
        LCG {
            displacement_intensity,
        }
    }

    pub fn lcg(&self) -> f64 {
        let displacement_intensity = self.displacement_intensity.spatium_functions();
        let mut moment = 0.0;
        let mut left_moment = 0.0;
        let mut right_moment = 0.0;
        let mut displacement_tonnage = 0.0;
        for spatium in displacement_intensity.as_ref() {
            let integral = spatium.integral();
            displacement_tonnage += integral;
            moment += integral * spatium.abscissa();
            if spatium.abscissa() < 0.0 {
                left_moment += integral * spatium.abscissa();
            } else if spatium.abscissa() > 0.0 {
                right_moment += integral * spatium.abscissa();
            }
        }
        moment / displacement_tonnage
    }
}
