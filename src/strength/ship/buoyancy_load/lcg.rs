use crate::strength::ship::displacement::displacement_intensity::DisplacementIntensity;




///
/// Абcцисса центр тяжести судна. Англ.: Longitudinal Center of Gravity (LCG).
/// Отсчитывается от мидель шпангоута. Имеет положительный знак от мидель шпангоута в нос судна.
pub struct LCG {
    shiploads: DisplacementIntensity

}

